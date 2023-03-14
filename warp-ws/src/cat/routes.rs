use std::sync::Arc;

use domains::{
    cat::models::{NewCat, ReplaceCat, UpdateCat},
    data_source::DataSource,
};

use warp::{Filter, Rejection, Reply};

use crate::helpers::{json_body, with_data};

use super::handlers;

pub fn routes_config(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let root = warp::path("cats");
    root.and(
        get_all(data.clone())
            .or(get_one(data.clone()))
            .or(post_one(data.clone()))
            .or(patch_one(data.clone()))
            .or(put_one(data.clone()))
            .or(delete_one(data.clone())),
    )
}

pub fn get_all(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!()
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::fetch_all)
}

pub fn get_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::get())
        .and(warp::any().map(move || data.clone()))
        .and_then(handlers::fetch_one)
}

pub fn post_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!()
        .and(warp::post())
        .and(with_data(data))
        .and(json_body::<NewCat>())
        .and_then(handlers::add_one)
}

pub fn patch_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::patch())
        .and(with_data(data))
        .and(json_body::<UpdateCat>())
        .and_then(handlers::modify_one)
}

pub fn put_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::put())
        .and(with_data(data))
        .and(json_body::<ReplaceCat>())
        .and_then(handlers::replace_one)
}

pub fn delete_one(
    data: Arc<DataSource>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!(i32)
        .and(warp::delete())
        .and(with_data(data))
        .and_then(handlers::remove_one)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::{InfoPayload, SuccessPayload};
    use domains::{
        cat::models::{Cat, CatId},
        data_source::{MockData, MockSource},
    };

    use super::*;

    fn test_data_mock() -> Arc<DataSource> {
        let data = MockSource::default().set(MockData::Cat(vec![
            Cat {
                id: CatId("1".into()),
                name: "A".into(),
                age: 1,
                weight: None,
                creation_time: Utc::now(),
            },
            Cat {
                id: CatId("2".into()),
                name: "B".into(),
                age: 1,
                weight: Some(3.0),
                creation_time: Utc::now(),
            },
        ]));
        Arc::new(DataSource::mock(Some(data)))
    }

    #[tokio::test]
    async fn test_get_all() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &get_all(data);

        // Act
        let res = warp::test::request().reply(reply_filter).await;
        let res_body = res.body();
        let payload: SuccessPayload<Vec<Cat>> = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.data.len(), 2);
    }

    #[tokio::test]
    async fn test_get_one() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &get_one(data);

        // Act
        let res = warp::test::request().path("/1").reply(reply_filter).await;
        let res_body = res.body();
        let payload: SuccessPayload<Cat> = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.data.id.0, "1".to_string());
    }

    #[tokio::test]
    async fn test_post_one() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &post_one(data);

        // Act
        let res = warp::test::request()
            .method("POST")
            .json(&NewCat {
                name: "C".into(),
                age: 2,
                weight: None,
            })
            .reply(reply_filter)
            .await;

        let res_body = res.body();
        let payload: SuccessPayload<Cat> = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.data.id.0, "3".to_string());
    }

    #[tokio::test]
    async fn test_patch_one() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &patch_one(data);

        // Act
        let res = warp::test::request()
            .method("PATCH")
            .path("/1")
            .json(&UpdateCat {
                name: None,
                age: Some(3),
                weight: Some(7.5),
            })
            .reply(reply_filter)
            .await;

        let res_body = res.body();
        let payload: SuccessPayload<Cat> = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.data.name, "A".to_string());
        assert_eq!(payload.data.weight.unwrap(), 7.5);
    }

    #[tokio::test]
    async fn test_put_one() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &put_one(data);

        // Act
        let res = warp::test::request()
            .method("PUT")
            .path("/1")
            .json(&ReplaceCat {
                name: "Z".into(),
                age: 5,
                weight: Some(5.4),
            })
            .reply(reply_filter)
            .await;

        let res_body = res.body();
        let payload: SuccessPayload<Cat> = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.data.name, "Z".to_string());
        assert_eq!(payload.data.age, 5);
        assert_eq!(payload.data.weight.unwrap(), 5.4);
    }

    #[tokio::test]
    async fn test_delete_one() {
        // Arrange
        let data = test_data_mock();
        let reply_filter = &delete_one(data);

        // Act
        let res = warp::test::request()
            .method("DELETE")
            .path("/2")
            .reply(reply_filter)
            .await;

        let res_body = res.body();
        let payload: InfoPayload = serde_json::from_slice(&res_body).unwrap();

        // Assert
        assert_eq!(payload.message.is_empty(), false);
    }
}
