use axum::{Extension, Json, extract::Path};
use sqlx::FromRow;
use serde::Serialize;

use crate::commands::add_command;

#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}

#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

pub async fn show_collectors(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<Collector>> {
    const SQL: &str = "SELECT 
    DISTINCT(id) AS id, 
    collector_id, 
    (SELECT MAX(received) FROM timeseries WHERE collector_id = ts.collector_id) AS last_seen 
    FROM timeseries ts";
    Json(sqlx::query_as::<_, Collector>(SQL)
        .fetch_all(&pool)
        .await
        .unwrap())
}

pub async fn collector_data(Extension(pool): Extension<sqlx::SqlitePool>, uuid: Path<String>) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received")
        .bind(uuid.as_str())
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(rows)
}

pub async fn shutdown_collector(uuid: Path<String>) {
    let uuid = uuid::Uuid::parse_str(uuid.as_str()).unwrap();
    let uuid = uuid.as_u128();
    add_command(uuid, shared_v3::TaskType::Shutdown);
}