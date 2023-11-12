use ethers::types::{Address, U256};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBody {
    pub src_token: String,
    pub src_decimals: Option<u8>,
    pub dest_token: String,
    pub dest_decimals: Option<u8>,
    pub src_amount: String,
    pub dest_amount: String,
    pub price_route: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage: Option<f64>,
    pub user_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_fee_bps: Option<String>,
    pub positive_slippage_to_user: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<u64>,
}

impl TransactionBody {
    pub fn new(
        src_token: Address,
        src_decimals: Option<u8>,
        dest_token: Address,
        dest_decimals: Option<u8>,
        src_amount: U256,
        dest_amount: U256,
        price_route: Value,
        sender: Address,
    ) -> Self {
        Self {
            src_token: format!("{:?}", src_token),
            src_decimals,
            dest_token: format!("{:?}", dest_token),
            dest_decimals,
            src_amount: src_amount.to_string(),
            dest_amount: dest_amount.to_string(),
            price_route,
            slippage: None,
            user_address: format!("{:?}", sender),
            tx_origin: None,
            receiver: None,
            partner_address: None,
            partner_fee_bps: None,
            positive_slippage_to_user: true,
            partner: None,
            permit: None,
            deadline: None,
        }
    }
}
