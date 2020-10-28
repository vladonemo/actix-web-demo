use std::env;

use actix_web::{Error, get, HttpResponse};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::diesel_demo::models::Post;

pub mod schema;
pub mod models;

const URL: &'static str = "DATABASE_URL";

fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var(URL).expect(&format!("{} must be set", URL));
    PgConnection::establish(&database_url).expect(&format!("Failed connecting to {}", database_url))
}

#[get("/posts")]
pub async fn get_posts() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Post::find_all(&connect())))
}