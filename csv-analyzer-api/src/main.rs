use axum::{
    Router,
    routing::{get, post},
};

use csv_analyzer_api::handlers::{
    csv_get_handler, csv_post_handler_with_json, csv_post_handler_with_query, health_handler,
    root_handler,
};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/csv", get(csv_get_handler))
        .route("/csv", post(csv_post_handler_with_json))
        .route("/csv_query", post(csv_post_handler_with_query))
        .route("/health", get(health_handler));

    let address = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
