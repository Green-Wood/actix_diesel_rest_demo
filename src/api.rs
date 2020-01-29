use crate::db::{db_all_posts, db_all_users, db_insert_post, db_insert_user, DBPool};
use crate::model::{NewPost, NewUser};
use actix_web::{web, HttpResponse};


pub async fn all_users(pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect("Fail to load db connection from  pool");
    let users = db_all_users(&conn);
    HttpResponse::Ok().json(users)
}

pub async fn insert_user(pool: web::Data<DBPool>, new_user: web::Json<NewUser>) -> HttpResponse {
    let conn = pool.get().expect("Fail to load db connection from  pool");

    let user = db_insert_user(&conn, &new_user);
    HttpResponse::Ok().json(user)
}

pub async fn all_posts(pool: web::Data<DBPool>) -> HttpResponse {
    let conn = pool.get().expect("Fail to load db connection from  pool");

    let posts = db_all_posts(&conn);
    HttpResponse::Ok().json(posts)
}

pub async fn insert_post(pool: web::Data<DBPool>, new_post: web::Json<NewPost>) -> HttpResponse {
    let conn = pool.get().expect("Fail to load db connection from  pool");

    let post = db_insert_post(&conn, &new_post);
    HttpResponse::Ok().json(post)
}
