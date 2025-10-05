#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsvSearchRequest {
    filename: Option<String>,
    uid: Option<String>,
}
