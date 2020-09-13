use actix_web::{delete, get, patch, post, Error, HttpResponse};

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
pub async fn create_recipe() -> Result<HttpResponse, Error> {
    unimplemented!()
}

#[delete("/recipes/{id}")]
pub async fn delete_recipe() -> Result<HttpResponse, Error> {
    unimplemented!()
}

#[patch("/recipes/{id}")]
pub async fn update_recipe() -> Result<HttpResponse, Error> {
    unimplemented!()
}
