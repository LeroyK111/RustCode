/*
! SQLite是一个c语言库，它实现了一个小型、快速、自包含、高可靠性、全功能的SQL数据库引擎，与Rust的安全性和性能非常匹配。

! sqlx crate是一个很棒的工具，可以提供到各种数据库(包括SQLite)的异步连接。sqlx的美妙之处在于它可以在编译时检查SQL查询语句以及它与Rust异步特性的兼容。
*/

use std::result::Result;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

// 传入db_url，它期望返回一个SqliteQueryResult
async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
    // 创建一个连接到db_url的连接池
    let pool = SqlitePool::connect(db_url).await?;

    // 定义数据库表
    let qry = 
    "PRAGMA foreign_keys = ON;
    CREATE TABLE IF NOT EXISTS settings
    (
        settings_id     INTEGER PRIMARY KEY NOT NULL,
        description     TEXT                NOT NULL,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        done            BOOLEAN             NOT NULL DEFAULT 0
    );
    CREATE TABLE IF NOT EXISTS project
    (
        project_id      INTEGER PRIMARY KEY AUTOINCREMENT,
        product_name    TEXT,
        created_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        updated_on      DATETIME DEFAULT    (datetime('now', 'localtime')),
        img_directory   TEXT     NOT NULL,
        out_directory   TEXT     NOT NULL,
        status          TEXT     NOT NULL,
        settings_id     INTEGER  NOT NULL DEFAULT 1,
        FOREIGN KEY (settings_id) REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
    );";

    // 运行
    let result = sqlx::query(qry).execute(&pool).await;

    // 关闭连接池
    pool.close().await; 

    result
}


#[async_std::main]
async fn main() {
    // 在项目根目录下创建一个数据库名为'sqlite.db'文件
    let db_url = String::from("sqlite://sqlite.db");

    // 如果数据库不存在，则创建它。
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();

        //如果存在，则调用create_schema
        match create_schema(&db_url).await {
            // 如果一切顺利，打印OK…否则panic
            Ok(_) => println!("database created succesfully"),
            Err(e) => panic!("{}", e)
        }
    }

    // 连接数据库
    let instances = SqlitePool::connect(&db_url).await.unwrap();

    // 在settings表的description字段插入"testing"
    let qry = "INSERT INTO settings (description) VALUES($1)";
    let result = sqlx::query(qry).bind("testing").execute(&instances).await;

    // 关闭数据库
    instances.close().await;
    println!("{:?}", result);
}


