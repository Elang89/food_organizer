// TODO: Remove macro use when support for new macro method is added
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod actions;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod resource;
pub mod schema;

use actix_web::{middleware, web, App, HttpServer};
use db::create_connection;
use dotenv::dotenv;
use handlers::recipe_handler;
use resource::ERROR_DB_ENV;
use std::{env, io};

pub mod embedded_migrations {
    use diesel::pg::PgConnection;
    use diesel_migrations::embed_migrations;

    embed_migrations!();

    pub fn run(connection: &PgConnection, db_name: &str) {
        embedded_migrations::run(connection)
            .expect(format!("Unable to run migrations on {}", db_name).as_str());
    }
}

#[actix_rt::main]
pub async fn run() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();

    let connection_url = env::var("DATABASE_URL").expect(ERROR_DB_ENV);
    let pool = create_connection(connection_url.as_str());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(recipe_handler::get_all_recipes)
                    .service(recipe_handler::create_recipe),
            )
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
