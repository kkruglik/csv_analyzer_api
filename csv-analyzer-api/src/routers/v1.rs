use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::handlers::{
    analyze_csv_by_uid_handler, analyze_csv_handler, csv_get_handler, csv_post_handler_with_json,
    csv_post_handler_with_query, health_handler, root_handler, upload_csv_handler,
};

pub async fn v1_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(root_handler))
        .route("/csv", get(csv_get_handler))
        .route("/csv", post(csv_post_handler_with_json))
        .route("/csv_query", post(csv_post_handler_with_query))
        .route("/csv/upload", post(upload_csv_handler))
        .route("/csv/analyze/{uid}", get(analyze_csv_by_uid_handler))
        .route("/health", get(health_handler))
        .route("/analyze", post(analyze_csv_handler))
}
