mod api_types;
mod token_price;

use actix_web::{
    web::{self, Query},
    App, HttpResponse, HttpServer, Responder,
};
use api_types::{
    ServiceResponse, TokenError, TokenPriceRequest, TokenPriceResponse, UNKNOWN_ERROR,
};
use once_cell::sync::Lazy;
use reqwest::Client;
use rust_decimal::{prelude::Signed, Decimal};

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

async fn get_token_price(query: web::Query<TokenPriceRequest>) -> impl Responder {
    let token_address = &query.token_address;
    let chain_id = &query.chain_id;
    let timestamp = query.timestamp;

    let result = input_validation(&query);
    match result {
        Ok(_) => match fetch_token_price(
            // safe to unwrap all of them here
            token_address.clone().unwrap().as_str(),
            chain_id.clone().unwrap().as_str(),
            timestamp.unwrap(),
        )
        .await
        {
            Ok(token_price_response) => HttpResponse::Ok().json(token_price_response),
            Err(err) => match err.error_code {
                10012 => HttpResponse::Unauthorized().json(err),
                404 => HttpResponse::NotFound().json(err),
                429 => HttpResponse::TooManyRequests().json(err),
                _ => HttpResponse::InternalServerError().json(err),
            },
        },
        Err(err) => err,
    }
}

fn input_validation(query: &Query<TokenPriceRequest>) -> Result<(), HttpResponse> {
    if query.token_address.is_none() || query.chain_id.is_none() || query.timestamp.is_none() {
        let err = if query.token_address.is_none() {
            TokenError {
                error_code: 404,
                error_message: "Token address is missing".to_string(),
            }
        } else if query.chain_id.is_none() {
            TokenError {
                error_code: 404,
                error_message: "Chain ID is missing".to_string(),
            }
        } else {
            TokenError {
                error_code: 404,
                error_message: "Timestamp is missing or invalid".to_string(),
            }
        };
        return Err(HttpResponse::NotFound().json(err));
    }
    Ok(())
}

async fn fetch_token_price(
    token_address: &str,
    chain_id: &str,
    timestamp: Decimal,
) -> Result<ServiceResponse, TokenError> {
    let token_price =
        token_price::fetch_token_price(&CLIENT, token_address, chain_id, timestamp).await?;

    prepare_response(token_price)
        .map(|(timestamp, token_price)| ServiceResponse {
            timestamp,
            chain_id: chain_id.to_string(),
            token_address: token_address.to_string(),
            token_price,
        })
        .ok_or_else(|| (*UNKNOWN_ERROR).clone())
}

fn prepare_response(token_price: TokenPriceResponse) -> Option<(Decimal, Decimal)> {
    token_price
        .prices
        .iter()
        .map(|vec| {
            (
                *vec.first().unwrap_or(&Decimal::ZERO),
                *vec.last().unwrap_or(&Decimal::ZERO),
            )
        })
        .min_by_key(|&(timestamp, _)| timestamp.abs_sub(&timestamp))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Token Price API. Use GET /token-price to query prices.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/token-price").route(web::get().to(get_token_price)))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
