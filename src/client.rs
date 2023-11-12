use anyhow::{Context, Result};
use reqwest::Client as HttpClient;

use crate::types::{
    PriceParams, PricePayload, Response, TransactionBody, TransactionPayload, TransactionQuery,
};

pub struct Client {
    base_url: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: "https://apiv5.paraswap.io".to_string(),
            http_client: HttpClient::new(),
        }
    }

    pub async fn price(&self, params: PriceParams) -> Result<PricePayload> {
        let price = self
            .http_client
            .get(format!("{}/prices", self.base_url))
            .query(&params)
            .send()
            .await
            .context("Failed to send price request to Paraswap")?
            .json::<Response<PricePayload>>()
            .await
            .context("Failed to deserialize Paraswap price response")?
            .try_into()?;

        Ok(price)
    }

    pub async fn transaction(
        &self,
        network: u64,
        query: TransactionQuery,
        body: TransactionBody,
    ) -> Result<TransactionPayload> {
        let transaction = self
            .http_client
            .post(format!("{}/transactions/{}", self.base_url, network))
            .query(&query)
            .json(&body)
            .send()
            .await
            .context("Failed to send transaction request to Paraswap")?
            .json::<Response<TransactionPayload>>()
            .await
            .context("Failed to deserialize Paraswap transaction response")?
            .try_into()?;

        Ok(transaction)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
