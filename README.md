# CSV Analyzer API

REST API for analyzing CSV files, providing statistical insights, type inference, and data quality metrics.

## Features

- **File Upload**: Accept CSV files via multipart/form-data
- **Automatic Type Inference**: Detect column types (Integer, Float, String, Boolean)
- **Statistical Analysis**: Calculate min, max, mean, and sum for numeric columns
- **Data Quality Metrics**: Track null counts for each column
- **Structured JSON Responses**: Clean, well-formatted API responses
- **Error Handling**: Proper HTTP status codes and error messages

## Requirements

- Rust 1.70+ (with 2024 edition support)
- Cargo

## Installation

```bash
git clone https://github.com/kkruglik/csv_analyzer_api.git
cd csv_analyzer_api/csv-analyzer-api
cargo build --release
```

## Running the Server

```bash
cargo run
```

The API will be available at `http://0.0.0.0:8000`

## API Endpoints

### `POST /api/v1/csv/upload`

Upload a CSV file and get a unique ID (file is stored in database).

**Request:**
- Content-Type: `multipart/form-data`
- Body: CSV file

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/csv/upload \
  -F "file=@sample.csv"
```

**Response:**

```json
{
  "uid": "6a126a9a-b3f8-44b9-8a12-b5a2e6ade68b",
  "filename": "sample.csv",
  "upload_datetime": "2025-10-10 14:30:45.123456 UTC"
}
```

### `GET /api/v1/csv/analyze/{uid}`

Analyze a previously uploaded CSV file by UID. Results are cached in the database.

**Example:**

```bash
curl http://localhost:8000/api/v1/csv/analyze/6a126a9a-b3f8-44b9-8a12-b5a2e6ade68b
```

**Response:** See full analysis response below.

### `POST /api/v1/analyze` (Legacy)

Upload and analyze a CSV file in one request.

**Request:**
- Content-Type: `multipart/form-data`
- Body: CSV file

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/analyze \
  -F "file=@sample.csv"
```

**Response:**

```json
{
  "filename": "sample.csv",
  "upload_datetime": "2025-10-05 15:19:39.494857 UTC",
  "uid": "52452611-3b8b-4b5f-8526-973b4887d576",
  "rows_count": 10,
  "columns_count": 8,
  "column_names": [
    "id",
    "name",
    "age",
    "salary",
    "department",
    "active",
    "start_date",
    "score"
  ],
  "columns_info": [
    {
      "column": "id",
      "dtype": "Integer",
      "max": 10,
      "mean": 5.5,
      "min": 1,
      "null_count": 0,
      "sum": 55
    },
    {
      "column": "name",
      "dtype": "Str",
      "max": null,
      "mean": null,
      "min": null,
      "null_count": 2,
      "sum": null
    },
    {
      "column": "age",
      "dtype": "Integer",
      "max": 42,
      "mean": 32.75,
      "min": 26,
      "null_count": 2,
      "sum": 262
    },
    {
      "column": "salary",
      "dtype": "Float",
      "max": 95000,
      "mean": 73428.79,
      "min": 58000.75,
      "null_count": 3,
      "sum": 514001.5
    }
  ]
}
```

### `GET /api/v1/health`

Health check endpoint.

**Example:**

```bash
curl http://localhost:8000/api/v1/health
```

**Response:**

```json
{
  "status": "ok"
}
```

## Sample CSV Format

```csv
id,name,age,salary,department,active,start_date,score
1,Alice Smith,28,75000.50,Engineering,true,2021-03-15,8.7
2,Bob Johnson,,65000,Marketing,false,2020-11-22,
3,Carol Davis,35,NA,Engineering,true,,9.2
```

The API handles:
- Missing values (empty cells)
- NA values
- Mixed data types
- Boolean values (true/false)
- Dates (as strings)

## Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `filename` | string | Name of the uploaded file |
| `upload_datetime` | string | Timestamp of upload (UTC) |
| `uid` | string | Unique identifier (UUID v4) |
| `rows_count` | number | Number of rows in the CSV |
| `columns_count` | number | Number of columns in the CSV |
| `column_names` | array | List of column headers |
| `columns_info` | array | Statistical analysis for each column |

### Column Info Fields

| Field | Type | Description |
|-------|------|-------------|
| `column` | string | Column name |
| `dtype` | string | Detected type: Integer, Float, Str, Boolean |
| `min` | number/null | Minimum value (numeric columns only) |
| `max` | number/null | Maximum value (numeric columns only) |
| `mean` | number/null | Average value (numeric columns only) |
| `sum` | number/null | Total sum (numeric columns only) |
| `null_count` | number | Number of missing values |

## Error Handling

| Status Code | Description |
|-------------|-------------|
| 400 | Bad Request - Invalid file, missing filename, or malformed CSV |
| 422 | Unprocessable Entity - CSV parsing error |
| 500 | Internal Server Error |

**Example Error Response:**

```bash
curl -X POST http://localhost:8000/api/v1/analyze
# Returns: "No file uploaded"
```

## Technology Stack

- **Framework**: Axum 0.8
- **Database**: SQLx with SQLite
- **CSV Processing**: csv_processor 0.1.10
- **Async Runtime**: Tokio
- **Serialization**: Serde + serde_json

## Database

Uses SQLite with automatic migrations on startup. Database file: `sqlite.db`

**Tables:**
- `csv_files` - Stores uploaded CSV files with raw content
- `csv_analyze_responses` - Stores analysis results (cached by file_id)

The database is automatically created and migrated on first run.

## Project Structure

```
csv-analyzer-api/
├── src/
│   ├── db/           # Database layer (repository, models)
│   ├── handlers/     # Request handlers
│   ├── models/       # Request/response models
│   ├── routers/      # API routing
│   ├── lib.rs
│   └── main.rs
├── migrations/       # SQLx migrations
├── Cargo.toml
└── README.md
```

## License

MIT
