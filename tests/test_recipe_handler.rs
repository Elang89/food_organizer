#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use food::handlers::recipe_handler;

    #[actix_rt::test]
    async fn test_get_all_recipes() {
        let mut app = test::init_service(
            App::new().service(web::scope("/api").service(recipe_handler::get_all_recipes)),
        )
        .await;
        let req = test::TestRequest::get().uri("/api/recipes").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success())
    }
}
