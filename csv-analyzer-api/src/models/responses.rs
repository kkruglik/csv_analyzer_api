#[derive(serde::Serialize, serde::Deserialize)]
pub struct CsvModel {
    pub filename: String,
    pub size: usize,
    pub upload_datetime: Option<String>,
    pub creator: Option<String>,
    pub uid: Option<String>,
}
