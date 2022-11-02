use actix_web::{get, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DomResponse {
    height: String,
    lon: String,
    lat: String
}

#[get("/")]
async fn dom() -> impl Responder {
    serde_json::to_string(&DomResponse {
        height: "4546 m ü. M.".to_string(),
        lon: "46° 5.64′ N 7° 51.53′ E ".to_string(),
        lat: "46° 5′ 38.4″ N 7° 51′ 31.8″ E".to_string(),
    }).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(dom)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
