use crate::constants;
use serde::{Deserialize, Serialize};
use ethers::{
    prelude::Address, 
    types::{
        H256, U256
    }
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Mainnet,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => constants::API_BASE_MAINNET,
        }
    }

    pub fn api(&self) -> String {
        let url = self.url();
        format!("{}{}{}", url, constants::API_PATH, constants::VERSION)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: Address,
    pub name: Option<String>,
    pub biography: Option<String>,
    pub website_link: Option<String>,
    pub instagram_link: Option<String>,
    pub twitter_link: Option<String>,
    pub is_verified: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub hash: String,
    pub collection_address: Address,
    pub token_id: U256,
    pub is_order_ask: bool,
    pub signer: Address,
    pub strategy: Address,
    pub currency_address: Address,
    pub amount: String,
    pub price: U256,
    pub nonce: String,
    pub start_time: u64,
    pub end_time: u64,
    pub min_percentage_to_ask: u64,
    pub params: String,
    pub status: String,
    pub signature: Option<String>,
    pub v: Option<u8>,
    pub r: Option<H256>,
    pub s: Option<H256>,
}

