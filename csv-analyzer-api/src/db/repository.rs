use crate::models::{ColumnInfo, CsvAnalyzeResponse};
use sqlx::Row;
use sqlx::SqlitePool;

pub async fn save_csv_file(
    pool: &SqlitePool,
    filename: &str,
    uid: &str,
    content: &str,
    upload_time: &str,
) -> Result<i64, sqlx::Error> {
    let query = "INSERT INTO csv_files (filename, upload_time, uid, content) VALUES (?, ?, ?, ?)";
    let result = sqlx::query(query)
        .bind(filename)
        .bind(upload_time)
        .bind(uid)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_csv_file_by_uid(
    pool: &SqlitePool,
    uid: &str,
) -> Result<(i64, String, String, String), sqlx::Error> {
    let query = "SELECT id, filename, content, upload_time FROM csv_files WHERE uid = ?";
    let row = sqlx::query(query).bind(uid).fetch_one(pool).await?;

    let id: i64 = row.get("id");
    let filename: String = row.get("filename");
    let content: String = row.get("content");
    let upload_time: String = row.get("upload_time");

    Ok((id, filename, content, upload_time))
}

pub async fn add_csv_to_db(
    pool: &SqlitePool,
    csv_data: &CsvAnalyzeResponse,
) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO csv_files (filename, upload_time, uid) VALUES (?, ?, ?)";
    sqlx::query(query)
        .bind(&csv_data.filename)
        .bind(&csv_data.upload_datetime)
        .bind(&csv_data.uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_csv_info_to_db(
    pool: &SqlitePool,
    file_id: i64,
    rows_count: usize,
    columns_count: usize,
    column_names: Vec<String>,
    columns_info: Vec<ColumnInfo>,
) -> Result<(), sqlx::Error> {
    let column_names_json =
        serde_json::to_string(&column_names).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let columns_info_json =
        serde_json::to_string(&columns_info).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    let insert_q = "INSERT INTO csv_analyze_responses (file_id, rows_count, columns_count, column_names, columns_info) VALUES (?, ?, ?, ?, ?)";

    sqlx::query(insert_q)
        .bind(file_id)
        .bind(rows_count as i32)
        .bind(columns_count as i32)
        .bind(&column_names_json)
        .bind(&columns_info_json)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_csv_analysis_by_uid(
    pool: &SqlitePool,
    uid: &str,
) -> Result<Option<CsvAnalyzeResponse>, sqlx::Error> {
    let query = "
        SELECT
            f.filename,
            f.upload_time,
            f.uid,
            a.rows_count,
            a.columns_count,
            a.column_names,
            a.columns_info
        FROM csv_files f
        LEFT JOIN csv_analyze_responses a ON f.id = a.file_id
        WHERE f.uid = ?
    ";

    let row = sqlx::query(query).bind(uid).fetch_optional(pool).await?;

    match row {
        Some(row) => {
            let rows_count_opt: Option<i32> = row.try_get("rows_count").ok();

            if rows_count_opt.is_none() {
                return Ok(None);
            }

            let column_names_json_opt: Option<String> = row.try_get("column_names").ok();
            let columns_info_json_opt: Option<String> = row.try_get("columns_info").ok();

            if column_names_json_opt.is_none() || columns_info_json_opt.is_none() {
                return Ok(None);
            }

            let column_names_json = column_names_json_opt.unwrap();
            let columns_info_json = columns_info_json_opt.unwrap();

            if column_names_json.is_empty() || columns_info_json.is_empty() {
                return Ok(None);
            }

            let filename: String = row.get("filename");
            let upload_datetime: String = row.get("upload_time");
            let uid: String = row.get("uid");
            let rows_count: i32 = row.get("rows_count");
            let columns_count: i32 = row.get("columns_count");

            let column_names: Vec<String> = serde_json::from_str(&column_names_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            let columns_info: Vec<ColumnInfo> = serde_json::from_str(&columns_info_json)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

            Ok(Some(CsvAnalyzeResponse {
                filename,
                upload_datetime,
                uid,
                rows_count: rows_count as usize,
                columns_count: columns_count as usize,
                column_names,
                columns_info,
            }))
        }
        None => Ok(None),
    }
}
