mod common;
use actix_web::{test, web, App};
use food_lib::db::create_connection;
use food_lib::handlers::recipe_handler;

#[actix_rt::test]
async fn test_get_all_recipes() {
    let _ctx = common::TestContext::new("postgres://root:password@localhost", "testdb");
    let mut app = test::init_service(
        App::new().service(web::scope("/api").service(recipe_handler::get_all_recipes)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/recipes").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_create_recipe() {
    let _ctx = common::TestContext::new("postgres://root:password@localhost", "testdb");
    let pool = create_connection("postgres://root:password@localhost/testdb");
    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .service(web::scope("/api").service(recipe_handler::create_recipe)),
    )
    .await;

    let req = test::TestRequest::post().uri("/api/recipes").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}
