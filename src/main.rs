#[macro_use]
extern crate diesel;

use crate::api::{all_posts, all_users, insert_post, insert_user};
use crate::db::{get_db_pool, DBPool};
use actix_web::{middleware, web, App, HttpServer};

mod api;
mod db;
mod model;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db_pool: DBPool = get_db_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .service(
                web::resource("/user")
                    .route(web::get().to(all_users))
                    .route(web::post().to(insert_user)),
            )
            .service(
                web::resource("/post")
                    .route(web::get().to(all_posts))
                    .route(web::post().to(insert_post)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
