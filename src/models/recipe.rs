use crate::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
