extern crate dotenv;

use dotenv::dotenv;
use std::env;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    dotenv().ok();

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&bind_address)
    .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
    .run()
    .unwrap();
}
