use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct TokenPriceResponse {
    pub(crate) prices: Vec<Vec<Decimal>>,
    pub(crate) market_caps: Vec<Vec<Decimal>>,
    pub(crate) total_volumes: Vec<Vec<Decimal>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokenPriceRequest {
    pub(crate) token_address: String,
    pub(crate) chain_id: String,
    pub(crate) timestamp: Decimal,
}

#[derive(Debug, Serialize)]
pub(crate) struct ServiceResponse {
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub(crate) timestamp: Decimal,
    pub(crate) chain_id: String,
    pub(crate) token_address: String,
    #[serde(serialize_with = "rust_decimal::serde::float::serialize")]
    pub(crate) token_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct TokenError {
    pub(crate) error_code: u16,
    pub(crate) error_message: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PlainError {
    pub(crate) error: String,
}

pub(crate) static UNKNOWN_ERROR: Lazy<TokenError> = Lazy::new(|| TokenError {
    error_code: 600,
    error_message: "Unknown error occurred".to_string(),
});

pub(crate) fn handle_error<E: std::fmt::Display>(error: E, code: u16, message: &str) -> TokenError {
    TokenError {
        error_code: code,
        error_message: format!("{}: {}", message, error),
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TokenErrorStatus {
    pub(crate) status: TokenError,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TokenErrorWrapper {
    pub(crate) error: TokenErrorStatus,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error occurred: [{}] {}",
            self.error_code, self.error_message
        )
    }
}
