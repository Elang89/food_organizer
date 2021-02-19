mod common;

use actix_web::{test, web, App};
use common::TEST_DB_URL;

use food_lib::db::create_connection;
use food_lib::handlers::recipe_handler;
use food_lib::models::ingredient::IngredientForRecipe;
use food_lib::models::recipe::{NewRecipeWithIngredients, Recipe};
use rstest::*;
use uuid::Uuid;

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

    let recipe = NewRecipeWithIngredients {
        name: "Ham Sandwich".to_string(),
        description: "Something".to_string(),
        ingredients: vec![
            IngredientForRecipe {
                id: Uuid::parse_str("390bf411-1aa7-48f4-a6fc-d161e0ddf63c").unwrap(),
                name: "cheese".to_string(),
            },
            IngredientForRecipe {
                id: Uuid::parse_str("9489cca0-3e7a-4571-9b58-96a4e3fe3e55").unwrap(),
                name: "ham".to_string(),
            },
        ],
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
