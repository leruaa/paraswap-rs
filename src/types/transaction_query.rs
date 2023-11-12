use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    pub ignore_checks: bool,
    pub ignore_gas_estimate: bool,
    pub eip1559: bool,
    pub only_params: bool,
}
