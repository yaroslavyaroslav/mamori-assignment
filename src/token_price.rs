use reqwest::Client;
use serde::{Deserialize, Serialize};

const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3/coins";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    id: String,
    symbol: String,
    current_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceRequest {
    pub token_address: String,
    pub chain_id: i64,
    pub timestamp: i64,
}

pub async fn fetch_token_price(
    client: &Client,
    token_address: &str,
    chain_id: i64,
    timestamp: i64,
) -> reqwest::Result<TokenPriceResponse> {
    let url = format!(
        "{}/{}?vs_currency=usd&chain_id={}&localization=false&x_currency=usd&order=market_cap_desc&per_page=100&page=1&sparkline=true&price_change_percentage=true&ids={}&time={}",
        COINGECKO_API_URL, token_address, chain_id, token_address, timestamp
    );
    let response = client.get(&url).send().await?;
    let json_response: TokenPriceResponse = response.json().await?;
    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_token_price() {
        let client = Client::new();
        let token_address = "bitcoin";
        let chain_id = 1;
        let timestamp = 1643723400;

        let token_price_response =
            fetch_token_price(&client, token_address, chain_id, timestamp).await;
        assert!(token_price_response.is_ok());
    }
}
