use crate::models::recipe::{NewRecipe, Recipe};
use crate::schema::recipes;
use crate::schema::recipes::all_columns;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn retrieve_recipes(
    conn: &PgConnection,
    limit: i64,
) -> Result<Vec<Recipe>, diesel::result::Error> {
    let result = recipes::table
        .select(all_columns)
        .limit(limit)
        .load::<Recipe>(conn)?;

    Ok(result)
}

pub fn insert_new_recipe(
    conn: &PgConnection,
    recipe: &NewRecipe,
) -> Result<Recipe, diesel::result::Error> {
    let result = diesel::insert_into(recipes::table)
        .values(recipe)
        .get_result::<Recipe>(conn)?;

    Ok(result)
}
