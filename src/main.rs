use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use actix_web::dev::Body;
use simd_json_derive::Serialize;

use crate::utils::to_body;

mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Serialize)]
pub struct Message {
    pub message: &'static str,
}

#[get("/json")]
async fn json() -> impl Responder {
    let message = Message {
        message: "Hello, World!",
    };

    let body = to_body(message);

    HttpResponse::Ok().body(Body::Bytes(body.freeze()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(json)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
