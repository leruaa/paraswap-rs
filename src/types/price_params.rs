use std::sync::Arc;

use erc20::Token;
use ethers::types::{Address, U256};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceParams {
    pub src_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_decimals: Option<u8>,
    pub dest_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_decimals: Option<u8>,
    pub amount: String,
    pub side: Side,
    pub network: u64,
    pub other_exchange_prices: bool,
    #[serde(rename = "includeDEXS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_dexs: Option<String>,
    #[serde(rename = "excludeDEXS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_dexs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_contract_methods: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_contract_methods: Option<String>,
    pub user_address: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_impact: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_token_transfert_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_token_transfert_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_token_dex_transfert_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dest_token_dex_transfert_fee: Option<String>,
}

impl PriceParams {
    pub fn new(
        from: &Arc<Token>,
        to: &Arc<Token>,
        amount: U256,
        side: Side,
        network: u64,
        user_address: Address,
    ) -> Self {
        Self {
            src_token: format!("{:?}", from.address),
            src_decimals: Some(from.decimals),
            dest_token: format!("{:?}", to.address),
            desc_decimals: Some(to.decimals),
            amount: format!("{amount:?}"),
            side,
            network,
            other_exchange_prices: false,
            include_dexs: None,
            exclude_dexs: None,
            include_contract_methods: None,
            exclude_contract_methods: None,
            user_address,
            receiver: None,
            route: None,
            partner: None,
            max_impact: None,
            src_token_transfert_fee: None,
            dest_token_transfert_fee: None,
            src_token_dex_transfert_fee: None,
            dest_token_dex_transfert_fee: None,
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    #[default]
    Sell,
    Buy,
}
