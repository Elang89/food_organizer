use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::ingredient::IngredientForRecipe;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewRecipeWithIngredients {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<IngredientForRecipe>,
}

#[derive(Debug, Insertable)]
#[table_name = "recipes_ingredients"]
pub struct RecipeIngredient {
    pub recipe_id: Uuid,
    pub ingredient_id: Uuid,
}
