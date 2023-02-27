use chrono::Utc;
use errors::{AppError, ClientError, Errors};

use crate::{
    account::models::{Account, AccountId},
    data_source::MockSource,
};

use super::models::SignUpAuth;

pub fn sign_up(sign_up_auth: SignUpAuth, source: &MockSource) -> Result<Account, AppError> {
    if sign_up_auth.password != sign_up_auth.confirmation {
        return Err(AppError::new(Errors::Client(ClientError::BadRequest {
            reason: "Password and confirmation don't match".into(),
        })));
    }

    let mut accounts = source.accounts.write().unwrap();

    let account_exist = accounts
        .clone()
        .into_iter()
        .position(|account| sign_up_auth.email == account.email);

    if account_exist.is_some() {
        return Err(AppError::new(Errors::Client(ClientError::Conflict {
            reason: "Account already existing for that email".into(),
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
