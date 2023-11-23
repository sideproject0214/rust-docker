use axum::{routing::get, Router};

use tower_http::cors::{Any, CorsLayer};
mod handlers;

#[tokio::main]
async fn main() {
    // tracing
    tracing_subscriber::fmt::init();

    // add cors
    let cors = CorsLayer::new().allow_origin(Any);

    // add postgres
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Error with pool connection");

    // add postgres table
    let _ = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS products(
                id serial,
                name text,
                price integer
               );"#,
    )
    .execute(&pool)
    .await;

    let app = Router::new()
        .route("/api", get(root))
        .route(
            "/api/products",
            get(handlers::get_products).post(handlers::create_product),
        )
        .route(
            "/api/products/:id",
            get(handlers::get_one_product)
                .delete(handlers::delete_product)
                .put(handlers::update_product),
        )
        .with_state(pool)
        .layer(cors);

    tracing::debug!("Listening on {}", "0.0.0.0:3000");

    println!("Listening on port {}", "0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Axum World!!!!"
}
