mod common;

use actix_web::{test, web, App};
use common::TEST_DB_URL;
use food_lib::db::create_connection;
use food_lib::handlers::recipe_handler;
use food_lib::models::recipe::{NewRecipe, Recipe};
use rstest::*;

#[rstest]
#[actix_rt::test]
async fn test_get_all_recipes() {
    let pool = create_connection(TEST_DB_URL);
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").service(recipe_handler::get_all_recipes)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/recipes").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());

    let result: Vec<Recipe> = test::read_body_json(resp).await;

    assert!(result.len() <= 50)
}

#[rstest]
#[actix_rt::test]
async fn test_create_recipe() {
    let pool = create_connection(TEST_DB_URL);
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").service(recipe_handler::create_recipe)),
    )
    .await;

    let recipe = NewRecipe {
        name: "Ham Sandwich".to_string(),
        description: "Something".to_string(),
    };

    let req = test::TestRequest::post()
        .set_json(&recipe)
        .uri("/api/recipes")
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[rstest]
#[actix_rt::test]
#[ignore]
async fn test_update_recipe() {}

#[rstest]
#[actix_rt::test]
#[ignore]
async fn test_delete_recipe() {}
