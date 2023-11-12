use ethers::types::{serde_helpers::StringifiedNumeric, Address, Bytes, U256};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPayload {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub data: Bytes,
    pub gas_price: Option<U256>,
    pub gas: Option<StringifiedNumeric>,
    pub chain_id: u64,
}
