use crate::models::CsvModel;
use axum::{Json, debug_handler, extract::Query};
use chrono::Utc;
use uuid::Uuid;

#[debug_handler]
pub async fn csv_get_handler() -> Json<CsvModel> {
    let resp = CsvModel {
        filename: "test.csv".to_string(),
        size: 100,
        upload_datetime: None,
        creator: Some("kirillkruglikov".to_string()),
        uid: None,
    };

    Json::from(resp)
}

pub async fn csv_post_handler_with_json(Json(mut v): Json<CsvModel>) -> Json<CsvModel> {
    v.uid = Some(Uuid::new_v4().to_string());
    v.upload_datetime = Some(Utc::now().to_string());
    Json::from(v)
}

pub async fn csv_post_handler_with_query(Query(mut v): Query<CsvModel>) -> Json<CsvModel> {
    v.uid = Some(Uuid::new_v4().to_string());
    v.upload_datetime = Some(Utc::now().to_string());
    Json::from(v)
}
