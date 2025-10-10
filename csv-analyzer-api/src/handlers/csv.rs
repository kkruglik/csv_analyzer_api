use super::utils::generate_csv_report;
use crate::db::{add_csv_info_to_db, get_csv_analysis_by_uid, get_csv_file_by_uid, save_csv_file};
use crate::models::{CsvAnalyzeResponse, CsvModel, CsvSearchRequest, CsvUploadResponse};
use axum::{
    Json, debug_handler,
    extract::{Multipart, Query},
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use sqlx::SqlitePool;
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

#[debug_handler]
pub async fn upload_csv_handler(
    State(pool): State<SqlitePool>,
    mut multipart: Multipart,
) -> Result<Json<CsvUploadResponse>, (StatusCode, String)> {
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

    let raw_csv = String::from_utf8_lossy(&data).to_string();

    let uid = Uuid::new_v4().to_string();
    let upload_datetime = Utc::now().to_string();

    save_csv_file(&pool, &filename, &uid, &raw_csv, &upload_datetime)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CsvUploadResponse {
        uid,
        filename,
        upload_datetime,
    }))
}

#[debug_handler]
pub async fn analyze_csv_by_uid_handler(
    State(pool): State<SqlitePool>,
    Path(uid): Path<String>,
) -> Result<Json<CsvAnalyzeResponse>, (StatusCode, String)> {
    if let Some(existing_analysis) = get_csv_analysis_by_uid(&pool, &uid).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("get_csv_analysis_by_uid error: {}", e),
        )
    })? {
        println!("DEBUG: Returning existing analysis for uid: {}", uid);
        return Ok(Json(existing_analysis));
    }

    let (file_id, filename, content, upload_datetime) = get_csv_file_by_uid(&pool, &uid)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, format!("CSV file not found: {}", e)))?;

    // Generate analysis report
    let (csv_report, rows_count, columns_count, column_names) = generate_csv_report(&content)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("generate_csv_report error: {}", e),
            )
        })?;

    println!(
        "DEBUG: csv_report length: {}, content: {}",
        csv_report.len(),
        &csv_report[..csv_report.len().min(200)]
    );

    let columns_info = serde_json::from_str(&csv_report).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "JSON parse error (line 96): {}. CSV report: '{}'",
                e,
                &csv_report[..csv_report.len().min(100)]
            ),
        )
    })?;

    // Save analysis to database
    add_csv_info_to_db(
        &pool,
        file_id,
        rows_count,
        columns_count,
        column_names.clone(),
        columns_info,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let columns_info_parsed =
        serde_json::from_str(&csv_report).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(CsvAnalyzeResponse {
        filename,
        upload_datetime,
        uid,
        rows_count,
        columns_count,
        column_names,
        columns_info: columns_info_parsed,
    }))
}

#[debug_handler]
pub async fn analyze_csv_handler(
    State(pool): State<SqlitePool>,
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

    let raw_csv = String::from_utf8_lossy(&data).to_string();

    let (csv_report, rows_count, columns_count, column_names) =
        generate_csv_report(&raw_csv).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let columns_info =
        serde_json::from_str(&csv_report).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let upload_datetime = Utc::now().to_string();
    let uid = Uuid::new_v4().to_string();

    // Save CSV file with content
    let file_id = save_csv_file(&pool, &filename, &uid, &raw_csv, &upload_datetime)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Save analysis
    add_csv_info_to_db(
        &pool,
        file_id,
        rows_count,
        columns_count,
        column_names.clone(),
        columns_info,
    )
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let columns_info_parsed =
        serde_json::from_str(&csv_report).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok(Json(CsvAnalyzeResponse {
        filename,
        upload_datetime,
        uid,
        rows_count,
        columns_count,
        column_names,
        columns_info: columns_info_parsed,
    }))
}
