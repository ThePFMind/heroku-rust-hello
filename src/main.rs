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

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("0.0.0.0", port))
    .unwrap_or_else(|_| panic!("Could not bind server to poty {}", &port))
    .run()
    .unwrap();
}
