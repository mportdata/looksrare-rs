use crate::constants;
use serde::{Deserialize, Serialize};
use ethers::{
    prelude::Address, 
    types::H256,
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
    pub token_id: String,
    pub is_order_ask: bool,
    pub signer: Address,
    pub strategy: Address,
    pub currency_address: Address,
    pub amount: String,
    pub price: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub address: Address,
    pub owner: Address,
    pub setter: Option<Address>,
    pub admin: Option<Address>,
    pub name: String,
    pub description: Option<String>,
    pub symbol: Option<String>,
    pub type_: String,
    pub website_link: Option<String>,
    pub facebook_link: Option<String>,
    pub twitter_link: Option<String>,
    pub instagram_link: Option<String>,
    pub telegram_link: Option<String>,
    pub medium_link: Option<String>,
    pub discord_link: Option<String>,
    pub is_verified: bool,
    pub is_explicit: bool,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    #[serde(rename = "bannerURI")]
    pub banner_uri: Option<String>,
}

