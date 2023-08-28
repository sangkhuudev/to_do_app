#[macro_use] extern crate diesel;
extern crate dotenv;

mod schema;
mod database;
mod to_do;
mod views;
mod serialization;
mod models;
mod auth;

use log;
use env_logger;
use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Either};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
    HttpServer::new(|| {
        App::new()
        // servoce => routing
        // req => service request
        .wrap_fn(|req, service| {
            let request_url: String = String::from(*&req.uri().path().clone());
            let passed: bool;
            if *&req.path().contains("/api/v1/item/") {
                match auth::process_token(&req) {
                    Ok(_token) => {
                        passed = true;
                    },
                    Err(_message) => {
                        passed = false;
                    }
                }
            } else {
                passed = true;
            }
            let end_result = match passed {
                true => {
                    Either::Left(service.call(req))
                },
                false => {
                    Either::Right(
                        ok(req.into_response(
                            HttpResponse::Unauthorized()
                            .finish()
                            .into_body()
                        ))
                    )
                }
            };
            async move {
                let result = end_result.await?;
                log::info!("{} -> {}", request_url, &result.status());
                Ok(result)
            }    
        })
        .configure(views::views_factory)
    })
    .bind("localhost:8000")?
    .run()
    .await
}