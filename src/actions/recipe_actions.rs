use crate::{
    models::queries::recipe_queries::SearchInfo,
    models::recipe::{NewRecipe, NewRecipeWithIngredients, Recipe, RecipeIngredient},
    schema::{recipes, recipes_ingredients},
};
use diesel::PgConnection;
use diesel::{prelude::*, query_dsl::methods::OrderDsl};

pub fn retrieve_recipes(
    conn: &PgConnection,
    search_info: &SearchInfo,
) -> Result<Vec<Recipe>, diesel::result::Error> {
    let mut query = recipes::table.into_boxed();

    query = match search_info.order_by.as_ref() {
        "created_at" => query.order_by(recipes::dsl::created_at),
        "name" => query.order_by(recipes::dsl::name),
        _ => query,
    };

    let result = query
        .select(recipes::all_columns)
        .limit(search_info.limit)
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
