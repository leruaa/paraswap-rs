use std::sync::Arc;

use erc20::mainnet::{USDT, WETH};
use ethers::types::{Address, U256};
use paraswap::{
    types::{PriceParams, Side},
    Client,
};

#[tokio::test]
async fn test_price() {
    let client = Client::new();
    let params = PriceParams::new(
        &Arc::new(WETH.clone()),
        &Arc::new(USDT.clone()),
        U256::from_dec_str("1000000000000000000").unwrap(),
        Side::Sell,
        1,
        Address::random(),
    );

    let price = client.price(params).await.unwrap();

    println!("{price:#?}")
}
