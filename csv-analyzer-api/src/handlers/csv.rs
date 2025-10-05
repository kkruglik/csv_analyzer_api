use crate::models::{CsvAnalyzeResponse, CsvModel, CsvSearchRequest};
use axum::{
    Json, debug_handler,
    extract::{Multipart, Query},
    http::StatusCode,
};
use chrono::Utc;
use csv_processor::DataFrame;
use csv_processor::reporter::generate_info_report;
use uuid::Uuid;

#[debug_handler]
pub async fn csv_get_handler(Query(r): Query<CsvSearchRequest>) -> Json<CsvModel> {
    let resp = CsvModel {
        filename: "test.csv".to_string(),
        size: 100,
        upload_datetime: None,
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

pub async fn analyze_csv_handler(
    mut multipart: Multipart,
) -> Result<Json<CsvAnalyzeResponse>, (StatusCode, String)> {
    let field = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
        .ok_or((StatusCode::BAD_REQUEST, "No file uploaded".to_string()))?;

    let filename = field
        .file_name()
        .ok_or((StatusCode::BAD_REQUEST, "Filename not found".to_string()))?
        .to_string();

    let data = field
        .bytes()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let raw_csv = String::from_utf8_lossy(&data);

    let df =
        DataFrame::from_strings(&raw_csv).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let csv_report = generate_info_report(&df)
        .to_json(csv_processor::JsonExportOrient::Records)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let columns_info =
        serde_json::from_str(&csv_report).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let upload_datetime = Utc::now().to_string();
    let uid = Uuid::new_v4().to_string();
    let (rows_count, columns_count) = df.shape();
    let column_names = df.headers().to_vec();
    let resp = CsvAnalyzeResponse {
        filename,
        upload_datetime,
        uid,
        rows_count,
        columns_count,
        columns_info,
        column_names,
    };

    Ok(Json(resp))
}
