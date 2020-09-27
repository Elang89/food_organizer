use crate::actions::recipe_actions::insert_new_recipe;
use crate::db::DbPool;
use crate::models::recipe::NewRecipe;
use actix_web::{delete, get, patch, post, web, Error, HttpResponse};

#[get("/recipes")]
pub async fn get_all_recipes() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json2(&"{\"message\": \"the message\"}")
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
    let conn = pool.get().expect("Could not get connection from pool");
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
