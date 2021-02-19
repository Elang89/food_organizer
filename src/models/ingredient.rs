use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct IngredientForRecipe {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
    pub description: String,
}
