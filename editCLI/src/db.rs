/*
db.rs：处理数据库连接和CRUD的操作
*/

use sqlx::Connection;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub hash: String,
    pub created_at: String,
}


async fn get_db() -> sqlx::Result<sqlx::SqliteConnection> {
    // 用于连接DATABASE_URL
    // 它可以是任何有效的SQLite连接字符串
    sqlx::SqliteConnection::connect(format!("sqlite:{}", std::env::var("KEY_STORE").unwrap()).as_str()).await
}

pub async fn connect() -> sqlx::SqliteConnection {
    // 如果不存在，则创建表
    let mut conn = get_db().await.expect("Error connecting to db");
    let _ = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL,
            value TEXT NOT NULL,
            hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    ).execute(&mut conn).await.unwrap();
    conn
}

pub async fn add(key: String, value: String, hash: String){
    let mut conn = connect().await;
        let _ = sqlx::query(
        r#"
        INSERT INTO entries (key, value, hash, created_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
        .bind(key)
        .bind(value)
        .bind(hash.clone())
        .bind(chrono::Local::now().to_rfc3339())
        .execute(&mut conn)
        .await.expect("Error adding entry");
}

pub async fn get(key: String) -> sqlx::Result<Entry> {
    let mut conn = connect().await;
    let row = sqlx::query("SELECT * FROM entries WHERE key = ?")
        .bind(key)
        .fetch_one(&mut conn)
        .await?;
    Ok(Entry {
        id: row.get(0),
        key: row.get(1),
        value: row.get(2),
        hash: row.get(3),
        created_at: row.get(4),
    })
}

pub async fn delete(key: String) -> sqlx::Result<()> {
    let mut conn = connect().await;
    sqlx::query("DELETE FROM entries WHERE key = ?")
        .bind(key)
        .execute(&mut conn)
        .await?;
    Ok(())
}

pub async fn list() -> sqlx::Result<Vec<Entry>> {
    let mut conn = connect().await;
    let mut entries = vec![];
    let rows = sqlx::query("SELECT * FROM entries")
        .fetch_all(&mut conn)
        .await?;
    for row in rows {
        entries.push(Entry {
            id: row.get(0),
            key: row.get(1),
            value: row.get(2),
            hash: row.get(3),
            created_at: row.get(4),
        });
    }
    Ok(entries)
}

pub async fn list_keys() -> sqlx::Result<Vec<String>> {
    let mut conn = connect().await;
    let mut keys = vec![];
    let rows = sqlx::query("SELECT key FROM entries")
        .fetch_all(&mut conn)
        .await?;
    for row in rows {
        keys.push(row.get(0));
    }
    Ok(keys)
}