# 探索Rust的ORM库

## Diesel

Diesel是一个强大的用于Rust的ORM和查询构建器，它强调安全性和易用性。以下是Diesel脱颖而出的原因：

类型安全：Diesel确保在编译时对SQL查询进行类型检查，从而显著减少运行时错误。


可扩展性：通过高级API，Diesel可以轻松的管理模式和构建查询。


多数据库支持：Diesel支持PostgreSQL、SQLite和MySQL，使其适用于各种应用程序。使用Diesel，可以无需手动管理数据库连接。它会帮你解决这些问题，让你专注于真正重要的事情——构建出色的应用程序！

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn establish_connection() -> PgConnection {
    let database_url = "postgres://username:password@localhost/mydb";
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

## SQLx：异步sql

SQLx是一个异步的纯rust编写的SQL工具包和ORM，它既强大又灵活。以下是它的一些主要特性：

异步支持：SQLx在构建时考虑了异步编程，因此非常适合高性能应用程序。


编译时检查：SQLx在编译时检查SQL查询，确保它们是正确的并减少运行时错误。


广泛的数据库支持：SQLx支持PostgreSQL, MySQL, SQLite和MSSQL。


SQLx的异步特性，加上其健壮的类型系统，使其成为现代Rust应用程序的绝佳选择。

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/mydb")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?;

    println!("Number of users: {}", row.0);
    Ok(())
}


## rusqlite

rusqlite是一个轻量级的符合人体工程学的库，用于在Rust中与SQLite数据库进行交互。以下是rusqlite很棒的原因：

简单：rusqlite的设计是直接的和易于使用的，使其完美的适合中小型应用程序。


Serde集成：它与Serde crate无缝集成，允许数据的类型可以安全的序列化和反序列化。


效率：围绕sqlite3 C库构建，rusqlite以最小的开销提供高效的数据库操作。


rusqlite非常适合只需要简单可靠的数据库解决方案，而不需要复杂性的ORM框架的应用程序。

use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("mydb.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;

    conn.execute(
        "INSERT INTO user (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, age FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for user in user_iter {
        println!("Found user {:?}", user?);
    }

    Ok(())
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    age: Option<i32>,
}