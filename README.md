# Token Retrieval API
## Overview
The Token Retrieval API is a Rust-based web service designed to fetch historical token prices from CoinGecko. It utilizes the Actix web framework to create a RESTful API that accepts GET requests to retrieve token prices.

## Purpose
The primary purpose of this API is to provide a simple and efficient way to fetch historical token prices for various blockchain chains and tokens. This can be useful for a variety of applications, including:

* Historical data analysis
* Price tracking
* Market data visualization

## Logic
The API logic is divided into two main components:

1. **API Endpoints**: The API exposes two endpoints:
    * `/token-price`: This endpoint accepts a GET request with query parameters `token_address`, `chain_id`, and `timestamp`. It returns the token price at the specified timestamp.
    * `/`: This endpoint returns a welcome message with instructions on how to use the API.
2. **Token Price Retrieval**: The `fetch_token_price` function is responsible for retrieving the token price from CoinGecko. It uses the `reqwest` library to send a GET request to the CoinGecko API with the provided `token_address`, `chain_id`, and `timestamp`. The response is then parsed into a `TokenPriceResponse` struct.

## Error Handling
The API uses a custom error handling system to handle errors that may occur during the token price retrieval process. The `handle_error` function is used to create a `TokenError` struct with a unique error code and message. The following error codes are used:

* `10012`: Request exceeds the allowed time range.
* `404`: Token not found.
* `429`: Rate limit exceeded.
* `600`: Unknown error occurred.

## How to Use
To use this API, follow these steps:

1. **Build the API**: Run `cargo build` to build the API.
2. **Start the API**: Run `cargo run` to start the API.
3. **Send a Request**: Use a tool like `curl` to send a GET request to the `/token-price` endpoint with the required query parameters. For example:
```bash
curl 'http://localhost:8080/token-price?token_address=0xdac17f958d2ee523a2206206994597c13d831ec7&chain_id=ethereum&timestamp=1643723400'
```
4. **Parse the Response**: The API will return a JSON response with the token price at the specified timestamp.

## API Types
The API uses several custom types to represent the request and response data. These types are defined in the `api_types` module and include:

* `TokenPriceRequest`: Represents the request data for the `/token-price` endpoint.
* `TokenPriceResponse`: Represents the response data from the CoinGecko API.
* `ServiceResponse`: Represents the response data returned by the API.
* `TokenError`: Represents an error that may occur during the token price retrieval process.

## Configuration
The API uses a single configuration variable to specify the CoinGecko API URL. This variable is defined in the `token_price` module as `COINGECKO_API_URL`.

## Testing
The API includes several tests to ensure that it is functioning correctly. These tests are defined in the `tests` module and cover the following scenarios:

* `test_too_old_fail_fetch_token_price`: Tests that the API returns an error when the request timestamp is too old.
* `test_wrong_contract_fail_fetch_token_price`: Tests that the API returns an error when the token contract is not found.
* `test_exceed_limit_fail_fetch_token_price`: Tests that the API returns an error when the rate limit is exceeded.
* `test_fetch_token_price`: Tests that the API returns the correct token price for a valid request.

## Dependencies
The API depends on the following crates:

* `actix-web`
* `reqwest`
* `rust_decimal`
* `serde`
* `once_cell`

## Contributing
Contributions to this API are welcome. Please submit a pull request with your changes and a clear explanation of what you have added or modified.

## License
This API is licensed under the MIT License. See the `LICENSE` file for more information.