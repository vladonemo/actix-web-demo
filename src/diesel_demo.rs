use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

const URL: &'static str = "DATABASE_URL";

fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var(URL).expect(&format!("{} must be set", URL));
    PgConnection::establish(&database_url).expect(&format!("Failed connecting to {}", database_url))
}