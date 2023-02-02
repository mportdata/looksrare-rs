use crate::types::{Account, Network, Order};
use thiserror::Error;
use ethers::{
    prelude::Address, 
    types::U256,
};
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
        Ok(data)
    }

    pub async fn get_orders(&self, req: OrdersRequest) -> Result<Vec<Order>, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/orders", api);


        let mut query = vec![];

        if let Some(_a) = &req.is_order_ask { query.push(("isOrderAsk", serde_json::to_value(req.is_order_ask)?)); };
        if let Some(_b) = &req.collection { query.push(("collection", serde_json::to_value(req.collection)?)); };
        if let Some(_c) = &req.token_id { query.push(("tokenId", serde_json::to_value(req.token_id)?)); };
        if let Some(_d) = &req.signer { query.push(("signer", serde_json::to_value(req.signer)?)); };
        if let Some(_e) = &req.nonce { query.push(("nonce", serde_json::to_value(req.nonce)?)); };
        if let Some(_f) = &req.strategy { query.push(("strategy", serde_json::to_value(req.strategy)?)); };
        if let Some(_g) = &req.currency { query.push(("currency", serde_json::to_value(req.currency)?)); };
        if let Some(_h) = &req.price { query.push(("price", serde_json::to_value(req.price)?)); };
        if let Some(_i) = &req.start_time { query.push(("startTime", serde_json::to_value(req.start_time)?)); };
        if let Some(_j) = &req.status { 
            &req.status.unwrap().iter_mut().for_each(|x| { query.push(("status[]", serde_json::to_value(x.to_str()).unwrap())) } ); 
        };
        if let Some(_k) = &req.pagination { query.push(("pagination", serde_json::to_value(req.pagination)?)); };
        if let Some(_l) = &req.sort { query.push(("sort", serde_json::to_value(req.sort)?)); };

        for (key, value) in &query {
            println!("{}: {}", key, value);
        }

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;
        println!("{}",text);
        let resp: OrdersResponse = serde_json::from_str(&text)?;
        println!("checkpoint test");
        let data: Option<Vec<Order>> = resp.data;

        let data_vec = match data {
            Some(i) => i,
            _ => vec![],
        };
        
        Ok(data_vec)
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrdersRequest {
    pub is_order_ask: Option<bool>,
    pub collection: Option<Address>,
    pub token_id: Option<U256>,
    pub signer: Option<Address>,
    pub nonce: Option<u64>,
    pub strategy: Option<Address>,
    pub currency: Option<Address>,
    pub price: Option<U256>,
    pub start_time: Option<u64>,
    pub status: Option<Vec<Status>>,
    pub pagination: Option<Pagination>,
    pub sort: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OrdersResponse {
    success: bool,
    message: Option<String>,
    data: Option<Vec<Order>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    first: u64,
    cursor: String,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Cancelled,
    Executed,
    Expired,
    Valid,
}

impl Status {
    fn to_str(&self) -> &str {
        match &self {
            Status::Cancelled => "CANCELLED",
            Status::Executed => "EXECUTED",
            Status::Expired => "EXPIRED",
            Status::Valid => "VALID",
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::types::Account;

    use super::*;

    #[tokio::test]
    async fn can_get_account() {
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

    #[tokio::test]
    async fn can_get_orders() {
        let api = LooksRareApi::new();

        let req = OrdersRequest {
            is_order_ask: None,
            collection: None,
            token_id: None,
            signer: Some("0x9E69b59b8d2A094CB1117f92Ff7DCf51Ed467B41".parse().unwrap()),
            nonce: None,
            strategy: None, 
            currency: None, 
            price: None, 
            start_time: None, 
            status: Some(vec![Status::Valid, Status::Expired]),
            pagination: None, 
            sort: None, 
        };
        
        let input_signer = req.signer;
        
        let orders: Vec<Order> = api.get_orders(req).await.unwrap();
        
        //let output_signer: = orders.signer;
        
        assert_eq!(1, 1);
    }
}

