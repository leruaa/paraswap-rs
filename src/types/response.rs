use serde::Deserialize;

use super::{price_payload::PricePayload, response_error::ResponseError, TransactionPayload};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(T),
    Error(ResponseError),
}

impl TryFrom<Response<PricePayload>> for PricePayload {
    type Error = ResponseError;

    fn try_from(value: Response<PricePayload>) -> Result<Self, Self::Error> {
        match value {
            Response::Ok(payload) => Ok(payload),
            Response::Error(err) => Err(err),
        }
    }
}

impl TryFrom<Response<TransactionPayload>> for TransactionPayload {
    type Error = ResponseError;

    fn try_from(value: Response<TransactionPayload>) -> Result<Self, Self::Error> {
        match value {
            Response::Ok(payload) => Ok(payload),
            Response::Error(err) => Err(err),
        }
    }
}
