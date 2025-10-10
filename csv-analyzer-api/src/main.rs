use axum::Router;
use csv_analyzer_api::handlers::handler_404;
use csv_analyzer_api::routers::v1_routes;
use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite://sqlite.db";

async fn create_main_router(pool: SqlitePool) -> Router {
    let v1_routes = v1_routes().await;
    Router::new()
        .nest("/api/v1", v1_routes)
        .fallback(handler_404)
        .with_state(pool)
}

async fn create_db() -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Database do not exists yet. Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await?
    }
    Ok(())
}

async fn create_db_pool() -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(DB_URL).await
}

async fn run_rb_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

#[tokio::main]
async fn main() {
    match create_db().await {
        Ok(()) => println!("Database create successfully"),
        Err(e) => panic!("Error while creating database {}", e),
    }
    let pool = {
        match create_db_pool().await {
            Ok(pool) => {
                println!("Database pool created successfully");
                pool
            }
            Err(e) => panic!("Error while creating database pool: {}", e),
        }
    };

    match run_rb_migrations(&pool).await {
        Ok(()) => println!("Migrations applied successfully"),
        Err(e) => panic!("Error while appling migrations: {}", e),
    }

    let router = create_main_router(pool).await;
    let address = "0.0.0.0:8000";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
