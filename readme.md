# TFT Price Oracle

Price oracle that fetches the TFT price and exposes a http server in order to retrieve the price remotely.

## Building

`cargo build`

## Running

`cargo run`

### Fetching the price:

`curl localhost:8000/`

## Implemented APIs

- https://min-api.cryptocompare.com/
- More to come soon