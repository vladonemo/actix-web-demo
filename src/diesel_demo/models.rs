use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::diesel_demo::schema::posts::dsl::posts;

use super::schema::posts as posts_table;

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

    pub fn add(connection: &PgConnection, post: InsertPost) -> i32 {
        diesel::insert_into(posts)
            .values(post)
            .returning(posts_table::id)
            .get_results::<i32>(connection)
            .unwrap()
            .first()
            .unwrap()
            .to_owned()
    }
}

#[derive(Insertable)]
#[table_name = "posts_table"]
pub struct InsertPost {
    pub title: String,
    pub body: String,
}
