use actix_web::{middleware, web, App, HttpServer};
use std::io;

pub mod actions;
pub mod error;
pub mod handlers;
pub mod models;

use handlers::recipe_handler;
#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").service(recipe_handler::get_all_recipes))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
