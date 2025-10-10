use csv_processor::{DataFrame, reporter::generate_info_report};

pub fn generate_csv_report(raw_csv: &str) -> Result<(String, usize, usize, Vec<String>), String> {
    let df = DataFrame::from_strings(&raw_csv).map_err(|e| e.to_string())?;
    let (rows_count, columns_count) = df.shape();
    let column_names = df.headers().to_vec();

    let csv_report = generate_info_report(&df)
        .to_json(csv_processor::JsonExportOrient::Records)
        .map_err(|e| e.to_string())?;
    Ok((csv_report, rows_count, columns_count, column_names))
}
