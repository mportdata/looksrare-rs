use crate::types::{Account, Network};
use thiserror::Error;
use ethers::prelude::Address;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

pub struct LooksRareApi {
    client: Client,
    network: Network,
}

impl LooksRareApi {
    pub fn new() -> Self {
        let builder = ClientBuilder::new();

        let client = builder.build().unwrap();

        Self {
            client,
            network: Network::Mainnet,
        }
    }

    pub async fn get_account(&self, req: AccountRequest) -> Result<Account, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/accounts", api);
        let mut map = std::collections::HashMap::new();
        map.insert("address", serde_json::to_value(req.address)?);

        let res = self.client.get(url).query(&map).send().await?;
        let text = res.text().await?;
        let resp: AccountResponse = serde_json::from_str(&text)?;
        let data: Account = resp.data.unwrap();
        println!("{}",text);
        Ok(data)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountRequest {
    pub address: Address,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AccountResponse {
    success: bool,
    message: Option<String>,
    data: Option<Account>,
}

#[derive(Debug, Error)]
pub enum LooksRareApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Order not found (address: {address}")]
    AccountNotFound { address: Address },
}

#[cfg(test)]
mod tests {
    use crate::types::Account;

    use super::*;

    #[tokio::test]
    async fn can_get_order() {
        let api = LooksRareApi::new();

        let req = AccountRequest {
            address: "0x3d67b76CF3dcc881255eb2262E788BE03b2f5B9F"
                .parse()
                .unwrap(),
        };
        let input_address = req.address;
        let account: Account = api.get_account(req).await.unwrap();
        let output_address: Address = account.address;
        assert_eq!(input_address, output_address);
    }
}

