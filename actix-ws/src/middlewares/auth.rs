use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{http, FromRequest, HttpRequest};

use errors::messages::AUTH_TOKEN_NOT_FOUND;
use errors::{AppError, ClientError, Errors};
use setup;

#[derive(Debug)]
pub struct JwtMiddleware {
    pub account_id: uuid::Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(AppError::new(Errors::Client(
                ClientError::Unauthorized {
                    reason: AUTH_TOKEN_NOT_FOUND.into(),
                },
            ))));
        }

        match setup::decode_claims(&token.unwrap()) {
            Ok(claims) => {
                let account_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
                ready(Ok(JwtMiddleware { account_id }))
            }
            Err(err) => {
                ready(Err(err))
            }
        }
    }
}
