#![crate_type = "lib"]

pub mod api;
pub mod constants;
pub mod types;

use api::{LooksRareApi, LooksRareApiError, AccountRequest};

use thiserror::Error;
use types::Account;
use ethers::prelude::Address;

pub async fn get_account(api: &LooksRareApi, address: Address) -> Result<Account, ClientError> {
    // get the account
    let req = AccountRequest {
        address: address,
    };

    let account = api
        .get_account(req)
        .await?;
    
    Ok(account)
}


#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    LooksRareApiError(#[from] LooksRareApiError),
}