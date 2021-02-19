use crate::models::recipe::{NewRecipe, NewRecipeWithIngredients, Recipe, RecipeIngredient};
use crate::schema::recipes::all_columns;
use crate::schema::{recipes, recipes_ingredients};
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
    recipe: &NewRecipeWithIngredients,
) -> Result<Recipe, diesel::result::Error> {
    let new_recipe = NewRecipe {
        name: recipe.name.clone(),
        description: recipe.description.clone(),
    };

    let result = diesel::insert_into(recipes::table)
        .values(new_recipe)
        .get_result::<Recipe>(conn)?;

    let recipes_ingredients = recipe
        .ingredients
        .iter()
        .map(|x| RecipeIngredient {
            recipe_id: result.id,
            ingredient_id: x.id,
        })
        .collect::<Vec<RecipeIngredient>>();

    diesel::insert_into(recipes_ingredients::table)
        .values(recipes_ingredients)
        .execute(conn)?;

    Ok(result)
}
