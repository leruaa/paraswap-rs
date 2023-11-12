use anyhow::Error;
use ethers::types::{serde_helpers::deserialize_stringified_numeric, Address, U256};
use serde::Deserialize;
use serde_json::Value;
use serde_this_or_that::as_f64;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricePayload {
    pub price_route: Value,
}

impl TryFrom<PricePayload> for PriceRoute {
    type Error = Error;

    fn try_from(value: PricePayload) -> std::result::Result<Self, Self::Error> {
        let price_route = serde_json::from_value(value.price_route)?;

        Ok(price_route)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceRoute {
    pub block_number: u64,
    pub network: u64,
    pub src_token: Address,
    pub src_decimals: u8,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub src_amount: U256,
    pub dest_token: Address,
    pub dest_decimals: u8,
    #[serde(deserialize_with = "deserialize_stringified_numeric")]
    pub dest_amount: U256,
    #[serde(rename = "srcUSD")]
    #[serde(deserialize_with = "as_f64")]
    pub src_usd: f64,
    #[serde(rename = "destUSD")]
    #[serde(deserialize_with = "as_f64")]
    pub dest_usd: f64,
    pub partner: String,
}
