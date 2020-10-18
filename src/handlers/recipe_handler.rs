use crate::actions::recipe_actions::{insert_new_recipe, retrieve_recipes};
use crate::db::DbPool;
use crate::models::recipe::NewRecipe;
use crate::resource::ERROR_DB_CONNECTION;

use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("/recipes")]
pub async fn get_all_recipes(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect(ERROR_DB_CONNECTION);
    let recipes = web::block(move || retrieve_recipes(&conn, 50))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(recipes))
}

#[get("/recipes/{id}")]
pub async fn get_recipe_by_id() -> Option<HttpResponse> {
    unimplemented!()
}

#[post("/recipes")]
pub async fn create_recipe(
    pool: web::Data<DbPool>,
    data: web::Json<NewRecipe>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect(ERROR_DB_CONNECTION);
    let recipe = web::block(move || insert_new_recipe(&conn, &data))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(recipe))
}

#[delete("/recipes/{id}")]
pub async fn delete_recipe() -> Result<HttpResponse, Error> {
    unimplemented!()
}

#[patch("/recipes/{id}")]
pub async fn update_recipe() -> Result<HttpResponse, Error> {
    unimplemented!()
}
