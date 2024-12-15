use crate::api_types::{
    handle_error, PlainError, TokenError, TokenErrorStatus, TokenErrorWrapper, TokenPriceResponse,
    UNKNOWN_ERROR,
};
use reqwest::Client;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3/coins";

pub async fn fetch_token_price(
    client: &Client,
    token_address: &str,
    chain_id: &str,
    timestamp: Decimal,
) -> Result<TokenPriceResponse, TokenError> {
    let url = format!(
        "{}/{}/contract/{}/market_chart/range?vs_currency=usd&chain_id={}&from={}&to={}",
        COINGECKO_API_URL,
        chain_id,
        token_address,
        chain_id,
        timestamp,
        timestamp + dec!(1000)
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| handle_error(e, 0000, "Request error"))?;

    if response.status().is_success() {
        let response_text = response
            .text()
            .await
            .map_err(|e| handle_error(e, 0000, "Decode error"))?;
        serde_json::from_str::<TokenPriceResponse>(&response_text)
            .map_err(|_| handle_error("JSON decode error", 0000, "Failed to decode JSON"))
    } else {
        let error_text = response
            .text()
            .await
            .map_err(|e| handle_error(e, 0000, "Decode error"))?;
        let decoded_error = serde_json::from_str::<TokenErrorWrapper>(&error_text)
            .map(|wrapper| wrapper.error.status)
            .or_else(|_| {
                serde_json::from_str::<PlainError>(&error_text).map(|e| TokenError {
                    error_code: 404,
                    error_message: e.error,
                })
            })
            .or_else(|_: serde_json::Error| {
                serde_json::from_str::<TokenErrorStatus>(&error_text).map(|status| status.status)
            })
            .unwrap_or((*UNKNOWN_ERROR).clone());
        Err(decoded_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static USDT_ADDRESS: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    static ETHEREUM_NETROWK: &str = "ethereum";

    #[tokio::test]
    async fn test_too_old_fail_fetch_token_price() {
        let client = Client::new();
        let token_address = USDT_ADDRESS;
        let chain_id = ETHEREUM_NETROWK;
        let timestamp = dec!(1643723400); // too old timestamp
        let error = TokenError {
            error_code: 10012,
            error_message: concat!(
                "Your request exceeds the allowed time range. ",
                "Public API users are limited to querying historical data within the past 365 days. ",
                "Upgrade to a paid plan to enjoy full historical data access: https://www.coingecko.com/en/api/pricing. "
            ).to_string()
        };

        let token_price_response =
            fetch_token_price(&client, token_address, chain_id, timestamp).await;
        match token_price_response {
            Err(e) => {
                assert_eq!(e.error_code, error.error_code);
                assert_eq!(e.error_message, error.error_message);
            }
            _ => panic!("Expected an error, but got a valid response"),
        }
    }

    #[tokio::test]
    async fn test_wrong_contract_fail_fetch_token_price() {
        let client = Client::new();
        let token_address = "0x111";
        let chain_id = ETHEREUM_NETROWK;
        let timestamp = dec!(1734260400);
        let error = TokenError {
            error_code: 404,
            error_message: "coin not found".to_string(),
        };

        let token_price_response =
            fetch_token_price(&client, token_address, chain_id, timestamp).await;
        match token_price_response {
            Err(e) => {
                assert_eq!(e.error_code, error.error_code);
                assert_eq!(e.error_message, error.error_message);
            }
            _ => panic!("Expected an error, but got a valid response"),
        }
    }

    #[tokio::test]
    async fn test_exceed_limit_fail_fetch_token_price() {
        let client = Client::new();
        let token_address = USDT_ADDRESS;
        let chain_id = ETHEREUM_NETROWK;
        let timestamp = dec!(1734260400);
        let error = TokenError {
            error_code: 429,
            error_message: concat!(
                "You've exceeded the Rate Limit. ",
                "Please visit https://www.coingecko.com/en/api/pricing to subscribe ",
                "to our API plans for higher rate limits."
            )
            .to_string(),
        };

        // exceeding the limit for a free plan intentionaly
        for _ in 0..10 {
            _ = fetch_token_price(&client, token_address, chain_id, timestamp).await;
        }

        let token_price_response =
            fetch_token_price(&client, token_address, chain_id, timestamp).await;

        match token_price_response {
            Err(e) => {
                assert_eq!(e.error_code, error.error_code);
                assert_eq!(e.error_message, error.error_message);
            }
            _ => panic!("Expected an error, but got a valid response"),
        }
    }

    #[tokio::test]
    async fn test_fetch_token_price() {
        let client = Client::new();
        let token_address = USDT_ADDRESS;
        let chain_id = ETHEREUM_NETROWK;
        let timestamp = dec!(1734260400);

        let token_price_response =
            fetch_token_price(&client, token_address, chain_id, timestamp).await;
        assert!(token_price_response.is_ok());
    }
}
