use crate::models::recipe::{NewRecipe, Recipe};
use crate::schema::recipes;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn insert_new_recipe(
    conn: &PgConnection,
    recipe: &NewRecipe,
) -> Result<Recipe, diesel::result::Error> {
    let result = diesel::insert_into(recipes::table)
        .values(recipe)
        .get_result::<Recipe>(conn)?;

    Ok(result)
}
