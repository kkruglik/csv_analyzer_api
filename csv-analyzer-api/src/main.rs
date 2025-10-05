use axum::Router;
use csv_analyzer_api::handlers::handler_404;
use csv_analyzer_api::routers::v1_routes;

async fn create_main_router() -> Router {
    let v1_routes = v1_routes().await;
    Router::new()
        .nest("/api/v1", v1_routes)
        .fallback(handler_404)
}

#[tokio::main]
async fn main() {
    let router = create_main_router().await;
    let address = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
