CREATE TABLE csv_files (
    id INTEGER PRIMARY KEY,
    filename TEXT NOT NULL,
    upload_time TEXT NOT NULL,
    uid TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL
);


CREATE TABLE csv_analyze_responses (
    id INTEGER PRIMARY KEY,
    rows_count INTEGER NOT NULL,
    columns_count INTEGER NOT NULL,
    column_names TEXT NOT NULL,
    columns_info TEXT NOT NULL,
    file_id INTEGER NOT NULL REFERENCES csv_files (id)
);
