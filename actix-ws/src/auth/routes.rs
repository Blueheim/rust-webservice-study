use actix_web::web::{self, ServiceConfig};

use super::handlers;

pub const SCOPE: &str = "/auth";

// routes
pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(SCOPE)
            .route("/signup/", web::post().to(handlers::sign_up))
            .route("/signin/", web::post().to(handlers::sign_in))
            .route("/signout/", web::get().to(handlers::sign_out)),
    );
}

#[cfg(test)]
mod tests {
    use dotenv;
    use std::str::FromStr;

    use actix_web::{test, App};
    use chrono::Utc;
    use common::{AuthPayload, SuccessPayload};
    use domains::{
        account::models::{Account, AccountId, SecureAccount},
        auth::models::{SignInAuth, SignUpAuth},
        data_source::{DataSource, MockData, MockSource},
    };

    use super::*;

    fn test_data_mock() -> web::Data<DataSource> {
        let data = MockSource::default().set(MockData::Account(vec![
            Account {
                id: AccountId::from_str("b8213d90-bfa5-43bd-a2d2-df94641f4176").unwrap(),
                email: "test@mail.com".into(),
                password: "$argon2id$v=19$m=4096,t=3,p=1$1t71JZJtA4E2y1+U0d6fNw$sJhlb1FYypxQ/268xg8V5JBsX0uGXFhWdu+WPRj7jz0".into(), // Pass:12345
                role: "member".into(),
                verified: false,
                creation_time: Utc::now(),
                last_modification_time: None,
            },
        ]));
        web::Data::new(DataSource::mock(Some(data)))
    }

    #[actix_web::test]
    async fn test_sign_up() {
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;
        let req = test::TestRequest::post()
            .uri(format!("{}/signup/", SCOPE).as_str())
            .set_json(SignUpAuth {
                email: "catlover@email.com".into(),
                password: "Yop?yop!123".into(),
                confirmation: "Yop?yop!123".into(),
            })
            .to_request();

        // Act
        let resp: SuccessPayload<SecureAccount> = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.data.email, "catlover@email.com".to_owned());
    }

    #[actix_web::test]
    async fn test_sign_in() {
        dotenv::dotenv().ok();
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;

        let req = test::TestRequest::post()
            .uri(format!("{}/signin/", SCOPE).as_str())
            .set_json(SignInAuth {
                email: "test@mail.com".into(),
                password: "Pass:12345".into(),
            })
            .to_request();

        // Act
        let resp: AuthPayload = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.token.is_some(), true);
    }

    #[actix_web::test]
    async fn test_sign_out() {
        dotenv::dotenv().ok();
        // Arrange
        let data = test_data_mock();
        let app =
            test::init_service(App::new().app_data(data.clone()).configure(routes_config)).await;

        let req = test::TestRequest::get()
            .uri(format!("{}/signout/", SCOPE).as_str())
            .to_request();

        // Act
        let resp: AuthPayload = test::call_and_read_body_json(&app, req).await;

        // Assert
        assert_eq!(resp.token.is_none(), true);
    }
}
