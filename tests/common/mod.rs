use diesel::pg::PgConnection;
use diesel::prelude::*;
use fake::faker::lorem::en::*;
use fake::Fake;
use food_lib::embedded_migrations;
use food_lib::models::recipe::{NewRecipe, Recipe};
use food_lib::schema::recipes;

pub struct TestContext {
    db_name: String,
    connection: PgConnection,
}

impl TestContext {
    pub fn new(base_url: &str, db_name: &str) -> Self {
        let postgres_url = format!("{}/postgres", base_url);

        let conn =
            PgConnection::establish(&postgres_url).expect("Could not connect to database, exiting");

        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());

        embedded_migrations::run(&conn, db_name);

        query
            .execute(&conn)
            .expect(format!("Could not create database {}, exiting", db_name).as_str());

        Self {
            db_name: db_name.to_string(),
            connection: conn,
        }
    }

    pub fn insert_recipe(&self) -> Result<Recipe, diesel::result::Error> {
        let name: String = Word().fake();
        let description: String = Paragraph(4..6).fake();

        let recipe = NewRecipe {
            name: name,
            description: description,
        };

        let result = diesel::insert_into(recipes::table)
            .values(&recipe)
            .get_result::<Recipe>(&self.connection)?;

        Ok(result)
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        let query = diesel::sql_query(format!("DROP DATABASE IF EXISTS {}", self.db_name).as_str());

        query
            .execute(&self.connection)
            .expect(format!("Failure dropping database {}, exiting", self.db_name).as_str());
    }
}
