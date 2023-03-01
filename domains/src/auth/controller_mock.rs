use std::env;

use chrono::Utc;
use errors::{messages, AppError, ClientError, Errors};
use setup::Claims;

use crate::{
    account::models::{Account, AccountId},
    data_source::MockSource,
};

use super::models::{SignInAuth, SignUpAuth};

pub async fn sign_up(sign_up_auth: SignUpAuth, source: &MockSource) -> Result<Account, AppError> {
    if sign_up_auth.password != sign_up_auth.confirmation {
        return Err(AppError::new(Errors::Client(ClientError::BadRequest {
            reason: messages::EMAIL_PASSWORD_INVALID.into(),
        })));
    }

    let mut accounts = source.accounts.write().unwrap();

    let account_exist = accounts
        .clone()
        .into_iter()
        .position(|account| sign_up_auth.email == account.email);

    if account_exist.is_some() {
        return Err(AppError::new(Errors::Client(ClientError::Conflict {
            reason: messages::ACCOUNT_ALREADY_EXISTING.into(),
        })));
    }

    let hashed_password = setup::hash_password(sign_up_auth.password);

    let account = Account {
        id: AccountId(uuid::Uuid::new_v4()),
        email: sign_up_auth.email,
        password: hashed_password,
        role: "member".into(),
        verified: false,
        creation_time: Utc::now(),
        last_modification_time: None,
    };

    accounts.push(account.to_owned());

    Ok(account)
}

pub async fn sign_in(sign_in_auth: SignInAuth, source: &MockSource) -> Result<String, AppError> {
    let accounts = source.accounts.read().unwrap();

    let existing_account = accounts
        .clone()
        .into_iter()
        .find(|account| sign_in_auth.email == account.email);

    if existing_account.is_none() {
        return Err(AppError::new(Errors::Client(ClientError::Conflict {
            reason: messages::EMAIL_PASSWORD_INVALID.into(),
        })));
    }

    let account = existing_account.unwrap();

    setup::verify_password(&account.password, sign_in_auth.password)?;

    let token = setup::encode_token(account.id.0.to_string())?;

    Ok(token)
}
