use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::diesel_demo::schema::posts::dsl::posts;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn find_all(connection: &PgConnection) -> Vec<Post> {
        posts.load(connection).unwrap()
    }
}
