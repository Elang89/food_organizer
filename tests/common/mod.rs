// use fake::faker::lorem::en::*;
// use fake::Fake;
// use food_lib::db::create_connection;
// use food_lib::models::recipe::{NewRecipe, Recipe};
// use food_lib::schema::recipes;

// const TEST_DB_NAME: &str = "testdb";
pub const TEST_DB_URL: &str = "postgres://root:password@localhost:5432/testdb";

// pub fn insert_recipe(&self) -> Result<Recipe, diesel::result::Error> {
//     let name: String = Word().fake();
//     let description: String = Paragraph(4..6).fake();

//     let recipe = NewRecipe {
//         name: name,
//         description: description,
//     };

//     let result = diesel::insert_into(recipes::table)
//         .values(&recipe)
//         .get_result::<Recipe>(&self.connection)?;

//     Ok(result)
// }
