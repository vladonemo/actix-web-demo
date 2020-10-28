use std::env;

use actix_web::{Error, get, post, HttpResponse};
use actix_web::web::Json;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::Deserialize;

use crate::diesel_demo::models::{Post, InsertPost};

pub mod schema;
pub mod models;

const URL: &'static str = "DATABASE_URL";

#[derive(Deserialize)]
pub struct NewPost {
    title: String,
    body: Option<String>,
}

fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var(URL).expect(&format!("{} must be set", URL));
    PgConnection::establish(&database_url).expect(&format!("Failed connecting to {}", database_url))
}

#[get("/posts")]
pub async fn get_posts() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Post::find_all(&connect())))
}

#[post("/posts")]
pub async fn add_posts(new_post: Json<NewPost>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Post::add(&connect(), InsertPost {
        title: new_post.title.to_string(),
        body: new_post.body.as_ref().unwrap_or(&"".to_owned()).to_string(),
    })))
}