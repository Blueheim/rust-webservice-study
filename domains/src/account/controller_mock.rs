use errors::{AppError, ClientError, Errors};

use crate::data_source::MockSource;

use super::models::Account;

pub async fn select_one(id: uuid::Uuid, source: &MockSource) -> Result<Account, AppError> {
    let accounts = source.accounts.read().unwrap();

    accounts
        .clone()
        .into_iter()
        .find(|account| id == account.id.0)
        .map_or_else(
            || {
                Err(AppError::new(Errors::Client(
                    ClientError::ResourceNotFound {
                        resource_name: "accounts".into(),
                        id: id.to_string(),
                    },
                )))
            },
            Ok,
        )
}
