#[macro_use]
extern crate rocket;
use reqwest;
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

#[get("/")]
async fn index(cl: &State<PriceClient>) -> Json<ApiResonse> {
    let usd = cl.get_price().await;
    Json(ApiResonse { usd })
}

#[launch]
fn rocket() -> _ {
    let price_client = PriceClient::default();
    tokio::spawn(price_client.clone().start());

    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);

    rocket::build()
        .mount("/", routes![index])
        .manage(price_client)
        .attach(cors.to_cors().unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResonse {
    #[serde(rename = "USD")]
    usd: f64,
}

#[derive(Clone, Default)]
struct PriceClient {
    price: Arc<RwLock<f64>>,
}

impl PriceClient {
    async fn get_price(&self) -> f64 {
        let price = self.price.read().await;
        return *price;
    }
    async fn start(self) {
        let mut interval = time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            self.set_price().await;
        }
    }
    async fn set_price(&self) {
        println!("calling set price");
        let res =
            reqwest::get("https://min-api.cryptocompare.com/data/price?fsym=3ft&tsyms=USD").await;

        match res {
            Ok(result) => {
                match result.status() {
                    reqwest::StatusCode::OK => match result.json::<ApiResonse>().await {
                        Ok(parsed) => {
                            let mut p = self.price.write().await;
                            *p = parsed.usd;
                        }
                        Err(_) => eprintln!("Hm, the response didn't match the shape we expected."),
                    },
                    _ =>  eprintln!("Request failed"),
                };
            }
            Err(err) => eprintln!("error getting price: {}", err),
        }
    }
}
