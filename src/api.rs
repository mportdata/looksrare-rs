use crate::types::{Account, CollectionInformation, CollectionRewards, CollectionStats, Network, Order};
use thiserror::Error;
use ethers::{
    prelude::Address, 
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

    pub async fn get_account(&self, address: Address) -> Result<Account, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/accounts", api);
        let mut map = std::collections::HashMap::new();
        map.insert("address", serde_json::to_value(address)?);

        let res = self.client.get(url).query(&map).send().await?;
        let text = res.text().await?;
        let resp: AccountResponse = serde_json::from_str(&text)?;
        let data: Account = resp.data.ok_or(LooksRareApiError::AccountNotFound {
            address: address
        })?;

        Ok(data)
    }

    pub async fn get_orders(&self, req: OrdersRequest) -> Result<Vec<Order>, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/orders", api);


        let mut query = vec![];

        if let Some(_a) = &req.is_order_ask { query.push(("isOrderAsk", serde_json::to_value(req.is_order_ask)?)); };
        if let Some(_b) = &req.collection { query.push(("collection", serde_json::to_value(req.collection)?)); };
        if let Some(_c) = &req.token_id { query.push(("tokenId", serde_json::to_value(req.token_id.unwrap().to_string())?)); };
        if let Some(_d) = &req.signer { query.push(("signer", serde_json::to_value(req.signer)?)); };
        if let Some(_e) = &req.nonce { query.push(("nonce", serde_json::to_value(req.nonce.unwrap().to_string())?)); };
        if let Some(_f) = &req.strategy { query.push(("strategy", serde_json::to_value(req.strategy)?)); };
        if let Some(_g) = &req.currency { query.push(("currency", serde_json::to_value(req.currency)?)); };
        if let Some(_h) = &req.price {
            if let Some(_min) = &req.price.clone().unwrap().min { query.push(("price[min]", serde_json::to_value(req.price.clone().unwrap().min.unwrap().to_string())?)); };
            if let Some(_max) = &req.price.clone().unwrap().max { query.push(("price[max]", serde_json::to_value(req.price.clone().unwrap().max.unwrap().to_string())?)); };
        };
        if let Some(_i) = &req.start_time { query.push(("startTime", serde_json::to_value(req.start_time.unwrap().to_string())?)); };
        if let Some(_j) = &req.end_time { query.push(("endTime", serde_json::to_value(req.end_time.unwrap().to_string())?)); };
        if let Some(_k) = &req.status { req.status.unwrap().iter_mut().for_each(|x| { query.push(("status[]", serde_json::to_value(x.to_str()).unwrap())) } ); };
        if let Some(_l) = &req.pagination {
            if let Some(_first) = &req.pagination.clone().unwrap().first { query.push(("pagination[first]", serde_json::to_value(req.pagination.clone().unwrap().first.unwrap().to_string())?)); };    
            if let Some(_cursor) = &req.pagination.clone().unwrap().cursor { query.push(("pagination[cursor]", serde_json::to_value(req.pagination.clone().unwrap().cursor)?)); }; 
        };
        if let Some(_m) = &req.sort { query.push(("sort", serde_json::to_value(req.sort.unwrap().to_str())?)); };

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;

        let resp: OrdersResponse = serde_json::from_str(&text)?;
        let data: Vec<Order> = resp.data.ok_or(LooksRareApiError::OrdersNotFound)?;

        Ok(data)
    }

    pub async fn get_nonce(&self, address: Address) -> Result<u64, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/orders/nonce", api);

        let mut query = vec![];
        query.push(("address", serde_json::to_value(address)?));

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;

        let resp: NonceResponse = serde_json::from_str(&text)?;
        let nonce_string: String = resp.data.ok_or(LooksRareApiError::NonceNotFound {
            address: address
        })?;
        let nonce: u64 = nonce_string.parse().unwrap();

        Ok(nonce)
    }

    pub async fn get_collection_information(&self, address:Address) -> Result<CollectionInformation, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/collections", api);

        let mut query = vec![];
        query.push(("address", serde_json::to_value(address)?));

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;

        let resp: CollectionInformationResponse = serde_json::from_str(&text)?;
        let collection_information: CollectionInformation = resp.data.ok_or(LooksRareApiError::CollectionNotFound{
            address: address
        })?;

        Ok(collection_information)
    }

    pub async fn get_collection_stats(&self, address:Address) -> Result<CollectionStats, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/collections/stats", api);

        let mut query = vec![];
        query.push(("address", serde_json::to_value(address)?));

        let res = self.client.get(url).query(&query).send().await?;
        let text = res.text().await?;
        let resp: CollectionStatsResponse = serde_json::from_str(&text)?;
        let collection_stats: CollectionStats = resp.data.ok_or(LooksRareApiError::CollectionNotFound {
            address: address
        })?;

        Ok(collection_stats)
    }

    pub async fn get_top_5_listing_rewards_collections(&self) -> Result<Vec<CollectionRewards>, LooksRareApiError> {
        let api = self.network.api();
        let url = format!("{}/collections/listing-rewards", api);

        let res = self.client.get(url).send().await?;
        let text = res.text().await?;
        println!("{}",text);
        let resp: Top5ListingRewardsCollectionsResponse = serde_json::from_str(&text)?;
        let top_5_listing_rewards_collections: Vec<CollectionRewards> = resp.data;

        Ok(top_5_listing_rewards_collections)
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
    pub token_id: Option<u64>,
    pub signer: Option<Address>,
    pub nonce: Option<u64>,
    pub strategy: Option<Address>,
    pub currency: Option<Address>,
    pub price: Option<Price>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub status: Option<Vec<Status>>,
    pub pagination: Option<Pagination>,
    pub sort: Option<Sort>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OrdersResponse {
    success: bool,
    message: Option<String>,
    data: Option<Vec<Order>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NonceResponse {
    success: bool,
    message: Option<String>,
    data: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CollectionInformationResponse {
    success: bool,
    message: Option<String>,
    data: Option<CollectionInformation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CollectionStatsResponse {
    success: bool,
    message: Option<String>,
    data: Option<CollectionStats>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Top5ListingRewardsCollectionsResponse {
    success: bool,
    message: Option<String>,
    data: Vec<CollectionRewards>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub first: Option<u64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Error)]
pub enum LooksRareApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Account not found (address: {address}")]
    AccountNotFound { address: Address },
    #[error("Orders not found")]
    OrdersNotFound,
    #[error("Nonce not found (address: {address}")]
    NonceNotFound { address: Address },
    #[error("Collection not found (address: {address}")]
    CollectionNotFound { address: Address },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Cancelled,
    Executed,
    Expired,
    Valid,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match &self {
            Status::Cancelled => "CANCELLED",
            Status::Executed => "EXECUTED",
            Status::Expired => "EXPIRED",
            Status::Valid => "VALID",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Sort {
    ExpiringSoon,
    Newest,
    PriceAsc,
    PriceDesc,
}

impl Sort {
    fn to_str(&self) -> &str {
        match &self {
            Sort::ExpiringSoon => "EXPIRING_SOON",
            Sort::Newest => "NEWEST",
            Sort::PriceAsc => "PRICE_ASC",
            Sort::PriceDesc => "PRICE_DESC",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    pub min: Option<u128>,
    pub max: Option<u128>,
}


#[cfg(test)]
mod tests {
    use crate::types::Account;

    use super::*;

    #[tokio::test]
    async fn can_get_account() {
        let api = LooksRareApi::new();

        let input_address: Address = "0x3d67b76CF3dcc881255eb2262E788BE03b2f5B9F".parse().unwrap();
        let account: Account = api.get_account(input_address).await.unwrap();
        let output_address: Address = account.address;
        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_orders() {
        let api = LooksRareApi::new();

        let req = OrdersRequest {
            is_order_ask: Some(true),
            collection: Some("0x34d85c9cdeb23fa97cb08333b511ac86e1c4e258".parse().unwrap()),
            token_id: Some(62962),
            signer: Some("0x9E69b59b8d2A094CB1117f92Ff7DCf51Ed467B41".parse().unwrap()),
            nonce: Some(17832),
            strategy: Some("0x579af6fd30bf83a5ac0d636bc619f98dbdeb930c".parse().unwrap()), 
            currency: Some("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse().unwrap()), 
            price: Some(Price{
                min: Some(12000000000000000000000000000000000000),
                max: Some(13000000000000000000000000000000000000),                
            }), 
            start_time: Some(1667747434), 
            end_time: Some(1667754634),
            status: Some(vec![Status::Cancelled,Status::Expired]),
            pagination: Some(Pagination {
                first: Some(4),
                cursor: None,
            }),
            sort: Some(Sort::Newest), 
        };
        
        let input_is_order_ask: bool = req.is_order_ask.unwrap();
        let input_collection: Address = req.collection.unwrap();
        let input_token_id: u64 = req.clone().token_id.unwrap();
        let input_signer: Address = req.signer.unwrap();
        let input_nonce: u64 = req.clone().nonce.unwrap();
        let input_strategy: Address = req.strategy.unwrap();
        let input_currency: Address = req.currency.unwrap();
        let input_min_price: u128 = req.price.clone().unwrap().min.unwrap();
        let input_max_price: u128 = req.price.clone().unwrap().max.unwrap();
        let input_start_time: u64 = req.start_time.unwrap();
        let input_end_time: u64 = req.end_time.unwrap();
        let input_status: Vec<Status> = req.clone().status.unwrap();

        let orders: Vec<Order> = api.get_orders(req).await.unwrap();
        let first_order: Order = orders.into_iter().nth(0).unwrap();

        let output_is_order_ask: bool = first_order.is_order_ask;
        let output_collection: Address = first_order.collection_address;
        let output_token_id: u64 = first_order.token_id.parse().unwrap();
        let output_signer: Address = first_order.signer;
        let output_nonce: u64 = first_order.nonce.parse().unwrap();
        let output_strategy: Address = first_order.strategy;
        let output_currency: Address = first_order.currency_address;
        let output_price: u128 = first_order.price.parse().unwrap();
        let output_start_time: u64 = first_order.start_time;
        let output_end_time: u64 = first_order.end_time;
        let output_status: String = first_order.status;

        assert_eq!(input_is_order_ask, output_is_order_ask);
        assert_eq!(input_collection, output_collection);
        assert_eq!(input_token_id, output_token_id);
        assert_eq!(input_signer, output_signer);
        assert_eq!(input_nonce, output_nonce);
        assert_eq!(input_strategy, output_strategy);
        assert_eq!(input_currency, output_currency);
        let greater_than_min_price: bool = input_min_price <= output_price;
        let less_than_max_price: bool = input_max_price >= output_price;
        assert!(greater_than_min_price && less_than_max_price);
        assert_eq!(input_start_time, output_start_time);
        assert_eq!(input_end_time, output_end_time);
        // test if output status is contained in list of input status
        assert!(input_status.iter().any(|i| i.to_str()==output_status));
    }

    #[tokio::test]
    async fn orders_pagination() {
        let api = LooksRareApi::new();

        let req = OrdersRequest {
            is_order_ask: None,
            collection: None,
            token_id: None,
            signer: None,
            nonce: None,
            strategy: None,
            currency: None, 
            price: None, 
            start_time: None, 
            end_time: None,
            status: None,
            pagination: Some(Pagination {
                first: Some(4),
                cursor: Some(String::from("0xd12240238374bbb1b23078fc71feeffa1d6c54b81888dfc5d9ea54d17c6a30a7")),
            }),
            sort: None, 
        };
        
        let input_pagination_first: usize = req.clone().pagination.unwrap().first.unwrap().try_into().unwrap();

        let orders: Vec<Order> = api.get_orders(req).await.unwrap();
        let orders_len: usize = orders.len();

        let output_pagination_first: usize = orders_len;

        assert_eq!(input_pagination_first, output_pagination_first);
    }

    #[tokio::test]
    async fn can_get_collection_information() {
        let api = LooksRareApi::new();

        let input_address: Address = "0x1A92f7381B9F03921564a437210bB9396471050C".parse().unwrap();
        
        let collection_information: CollectionInformation = api.get_collection_information(input_address).await.unwrap();
       
        let output_address: Address = collection_information.address;

        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_collection_stats() {
        let api = LooksRareApi::new();

        let input_address: Address = "0x1A92f7381B9F03921564a437210bB9396471050C".parse().unwrap();
        
        let collection_stats: CollectionStats = api.get_collection_stats(input_address).await.unwrap();
       
        let output_address: Address = collection_stats.address;

        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_top_5_listing_rewards_collections() {
        let api = LooksRareApi::new();
        
        let top_5_listing_rewards_collections: Vec<CollectionRewards> = api.get_top_5_listing_rewards_collections().await.unwrap();

        let num_of_collections: usize = top_5_listing_rewards_collections.len();
        println!("{}", num_of_collections);
        assert_eq!(num_of_collections, 5);
    }
}

