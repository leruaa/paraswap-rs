use std::sync::Arc;

use erc20::mainnet::{USDT, WETH};
use ethers::types::U256;
use paraswap::{
    types::{PriceParams, PriceRoute, Side, TransactionBody, TransactionQuery},
    Client,
};

#[tokio::test]
async fn test_transaction() {
    let client = Client::new();
    let from_token = Arc::new(WETH.clone());
    let to_token = Arc::new(USDT.clone());
    let from_amount = U256::from_dec_str("1000000000000000000").unwrap();
    let sender = "0x0059efca9d24bf579EE0314e7BC2569674aA42E2"
        .parse()
        .unwrap();
    let params = PriceParams::new(&from_token, &to_token, from_amount, Side::Sell, 1, sender);

    let price = client.price(params).await.unwrap();
    let price_route = PriceRoute::try_from(price.clone()).unwrap();

    let query = TransactionQuery {
        gas_price: None,
        ignore_checks: true,
        ignore_gas_estimate: true,
        eip1559: true,
        only_params: false,
    };

    let body = TransactionBody::new(
        from_token.address,
        Some(from_token.decimals),
        to_token.address,
        Some(to_token.decimals),
        from_amount,
        price_route.dest_amount,
        price.price_route.clone(),
        sender,
    );

    let transaction = client.transaction(1, query, body).await;

    println!("{transaction:#?}")
}
