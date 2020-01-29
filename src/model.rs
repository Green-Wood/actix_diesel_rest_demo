use crate::schema::{posts, users};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}
