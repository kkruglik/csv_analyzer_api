#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsvModel {
    pub filename: String,
    pub size: usize,
    pub upload_datetime: Option<String>,
    pub uid: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsvUploadResponse {
    pub uid: String,
    pub filename: String,
    pub upload_datetime: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsvAnalyzeResponse {
    pub filename: String,
    pub upload_datetime: String,
    pub uid: String,
    pub rows_count: usize,
    pub columns_count: usize,
    pub column_names: Vec<String>,
    pub columns_info: Vec<ColumnInfo>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ColumnInfo {
    pub column: String,
    pub dtype: String,
    pub max: Option<f64>,
    pub mean: Option<f64>,
    pub min: Option<f64>,
    pub null_count: usize,
    pub sum: Option<f64>,
}
