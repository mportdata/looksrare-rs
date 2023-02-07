#![crate_type = "lib"]

pub mod api;
pub mod constants;
pub mod types;

use api::{
    LooksRareApi, 
    LooksRareApiError, 
    OrdersRequest,
    Pagination,
    Price,
    Sort,
    Status,
};

use types::{
    Account, 
    CollectionInformation,
    CollectionStats,
    Order,
};

use thiserror::Error;
use ethers::prelude::Address;

pub async fn get_account(
    api: &LooksRareApi, 
    address: Address
) -> Result<Account, ClientError> {
    // get the account

    let account = api
        .get_account(address)
        .await?;
    
    Ok(account)
}

pub async fn get_orders(
    api: &LooksRareApi, 
    is_order_ask: Option<bool>,
    collection: Option<Address>,
    token_id: Option<u64>,
    signer: Option<Address>,
    nonce: Option<u64>,
    strategy: Option<Address>,
    currency: Option<Address>,
    min_price: Option<u128>,
    max_price: Option<u128>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    status: Option<Vec<Status>>,
    num_of_orders: Option<u64>,
    start_after_hash: Option<String>,
    sort: Option<Sort>,
) -> Result<Vec<Order>, ClientError> {
    
    let req = OrdersRequest {
        is_order_ask: is_order_ask,
        collection: collection,
        token_id: token_id,
        signer: signer,
        nonce: nonce,
        strategy: strategy,
        currency: currency,
        price: Some(Price{ min: min_price, max: max_price}),
        start_time: start_time,
        end_time: end_time,
        status: status,
        pagination: Some(Pagination { first: num_of_orders, cursor: start_after_hash}),
        sort: sort,
    };

    let orders = api
        .get_orders(req)
        .await?;

    Ok(orders)
}

pub async fn get_nonce(
    api: &LooksRareApi, 
    address: Address,
) -> Result<u64, ClientError> {
    // get the account

    let nonce = api
        .get_nonce(address)
        .await?;
    
    Ok(nonce)
}

pub async fn get_collection_information(
    api: &LooksRareApi,
    address: Address,
) -> Result<CollectionInformation, ClientError> {
    let collection_information = api
        .get_collection_information(address)
        .await?;

    Ok(collection_information)
}

pub async fn get_collection_stats(
    api: &LooksRareApi,
    address: Address,
) -> Result<CollectionStats, ClientError> {
    let collection_stats = api
        .get_collection_stats(address)
        .await?;

    Ok(collection_stats)
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    LooksRareApiError(#[from] LooksRareApiError),
}

#[cfg(test)]
mod tests {
    //use crate::types::Account;

    use super::*;

    #[tokio::test]
    async fn can_get_account() {
        let api = LooksRareApi::new();
        let input_address: Address = "0x3d67b76CF3dcc881255eb2262E788BE03b2f5B9F".parse().unwrap();
        let account: Account = get_account(&api, input_address).await.unwrap();
        let output_address: Address = account.address;
        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_orders() {
        let api = LooksRareApi::new();
        let input_is_order_ask: Option<bool> = Some(true);
        let input_collection: Option<Address> = Some("0x34d85c9cdeb23fa97cb08333b511ac86e1c4e258".parse().unwrap());
        let input_token_id: Option<u64> = Some(62962);
        let input_signer: Option<Address> = Some("0x9E69b59b8d2A094CB1117f92Ff7DCf51Ed467B41".parse().unwrap());
        let input_nonce: Option<u64> = Some(17832);
        let input_strategy: Option<Address> = Some("0x579af6fd30bf83a5ac0d636bc619f98dbdeb930c".parse().unwrap());
        let input_currency: Option<Address> = Some("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".parse().unwrap());
        let input_min_price: Option<u128> = Some(12000000000000000000000000000000000000);
        let input_max_price: Option<u128> = Some(13000000000000000000000000000000000000);                
        let input_start_time: Option<u64> = Some(1667747434);
        let input_end_time:Option<u64> = Some(1667754634);
        let input_status: Option<Vec<Status>> = Some(vec![Status::Cancelled, Status::Expired]);
        let input_num_of_orders: Option<u64> = Some(4);
        let input_start_after_hash: Option<String> = None;
        let input_sort: Option<Sort> = Some(Sort::Newest);

        let orders: Vec<Order> = get_orders(
            &api,
            input_is_order_ask,
            input_collection,
            input_token_id,
            input_signer,
            input_nonce,
            input_strategy,
            input_currency,
            input_min_price,
            input_max_price,
            input_start_time,
            input_end_time,
            input_status.clone(),
            input_num_of_orders,
            input_start_after_hash,
            input_sort,
        ).await.unwrap();

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

        assert_eq!(input_is_order_ask.unwrap(), output_is_order_ask);
        assert_eq!(input_collection.unwrap(), output_collection);
        assert_eq!(input_token_id.unwrap(), output_token_id);
        assert_eq!(input_signer.unwrap(), output_signer);
        assert_eq!(input_nonce.unwrap(), output_nonce);
        assert_eq!(input_strategy.unwrap(), output_strategy);
        assert_eq!(input_currency.unwrap(), output_currency);
        let greater_than_min_price: bool = input_min_price.unwrap() <= output_price;
        let less_than_max_price: bool = input_max_price.unwrap() >= output_price;
        assert!(greater_than_min_price && less_than_max_price);
        assert_eq!(input_start_time.unwrap(), output_start_time);
        assert_eq!(input_end_time.unwrap(), output_end_time);
        // test if output status is contained in list of input status
        assert!(input_status.unwrap().iter().any(|i| i.to_str()==output_status));
    }

    #[tokio::test]
    async fn can_get_collection_information() {
        let api = LooksRareApi::new();
        let input_address: Address = "0x1A92f7381B9F03921564a437210bB9396471050C".parse().unwrap();
        let collection_information: CollectionInformation = get_collection_information(&api, input_address).await.unwrap();
        let output_address: Address = collection_information.address;
        assert_eq!(input_address, output_address);
    }

    #[tokio::test]
    async fn can_get_collection_stats() {
        let api = LooksRareApi::new();
        let input_address: Address = "0x1A92f7381B9F03921564a437210bB9396471050C".parse().unwrap();
        let collection_stats: CollectionStats = get_collection_stats(&api, input_address).await.unwrap();
        let output_address: Address = collection_stats.address;
        assert_eq!(input_address, output_address);
    }
}