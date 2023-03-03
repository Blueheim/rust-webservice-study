use actix_web::web::{self, ServiceConfig};

use super::handlers;

pub const SCOPE: &str = "/cats";

// routes
pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(SCOPE)
            .route("/", web::get().to(handlers::fetch_all_cats))
            .route("/{cat_id}/", web::get().to(handlers::fetch_one_cat))
            .route("/", web::post().to(handlers::add_new_cat))
            .route("/{cat_id}/", web::patch().to(handlers::modify_cat))
            .route("/{cat_id}/", web::put().to(handlers::replace_cat))
            .route("/{cat_id}/", web::delete().to(handlers::remove_cat)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{test, web, App};
    use chrono::{Utc};
    use common::{InfoPayload, SuccessPayload};
    use domains::{
        cat::models::{Cat, CatId, NewCat, ReplaceCat, UpdateCat},
        data_source::{DataSource, MockData, MockSource},
    };

    fn test_data_mock() -> web::Data<DataSource> {
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
        web::Data::new(DataSource::mock(Some(data)))
    }

    #[actix_web::test]
    async fn test_get_all_cats() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::get()
            .uri(format!("{}/", SCOPE).as_str())
            .to_request();

        // Act
        let resp: SuccessPayload<Vec<Cat>> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.len(), 2);
    }

    #[actix_web::test]
    async fn test_get_one_cat() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::get()
            .uri(format!("{}/1/", SCOPE).as_str())
            .to_request();

        // Act
        let resp: SuccessPayload<Cat> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.id.0, "1".to_string());
    }

    #[actix_web::test]
    async fn test_post_cat() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::post()
            .uri(format!("{}/", SCOPE).as_str())
            .set_json(NewCat {
                name: "C".into(),
                age: 2,
                weight: None,
            })
            .to_request();

        // Act
        let resp: SuccessPayload<Cat> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.id.0, "3".to_string());
    }

    #[actix_web::test]
    async fn test_patch_cat() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::patch()
            .uri(format!("{}/1/", SCOPE).as_str())
            .set_json(UpdateCat {
                name: None,
                age: Some(3),
                weight: Some(7.5),
            })
            .to_request();

        // Act
        let resp: SuccessPayload<Cat> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.name, "A".to_string());
        assert_eq!(resp.data.weight.unwrap(), 7.5);
    }

    #[actix_web::test]
    async fn test_put_cat() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::put()
            .uri(format!("{}/1/", SCOPE).as_str())
            .set_json(ReplaceCat {
                name: "Z".into(),
                age: 5,
                weight: Some(5.4),
            })
            .to_request();

        // Act
        let resp: SuccessPayload<Cat> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.name, "Z".to_string());
        assert_eq!(resp.data.age, 5);
        assert_eq!(resp.data.weight.unwrap(), 5.4);
    }

    #[actix_web::test]
    async fn test_delete_cat() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::delete()
            .uri(format!("{}/2/", SCOPE).as_str())
            .to_request();

        // Act
        let resp: InfoPayload = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.message.is_empty(), false);
    }
}
