# 探索Rust的ORM库

## Diesel

Diesel是一个强大的用于Rust的ORM和查询构建器，它强调安全性和易用性。以下是Diesel脱颖而出的原因：

类型安全：Diesel确保在编译时对SQL查询进行类型检查，从而显著减少运行时错误。


可扩展性：通过高级API，Diesel可以轻松的管理模式和构建查询。


多数据库支持：Diesel支持PostgreSQL、SQLite和MySQL，使其适用于各种应用程序。使用Diesel，可以无需手动管理数据库连接。它会帮你解决这些问题，让你专注于真正重要的事情——构建出色的应用程序！
```rust
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

fn establish_connection() -> PgConnection {
    let database_url = "postgres://username:password@localhost/mydb";
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

```

## SQLx：异步sql

SQLx是一个异步的纯rust编写的SQL工具包和ORM，它既强大又灵活。以下是它的一些主要特性：

异步支持：SQLx在构建时考虑了异步编程，因此非常适合高性能应用程序。


编译时检查：SQLx在编译时检查SQL查询，确保它们是正确的并减少运行时错误。


广泛的数据库支持：SQLx支持PostgreSQL, MySQL, SQLite和MSSQL。


SQLx的异步特性，加上其健壮的类型系统，使其成为现代Rust应用程序的绝佳选择。
```rust
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

```


## rusqlite

rusqlite是一个轻量级的符合人体工程学的库，用于在Rust中与SQLite数据库进行交互。以下是rusqlite很棒的原因：

简单：rusqlite的设计是直接的和易于使用的，使其完美的适合中小型应用程序。


Serde集成：它与Serde crate无缝集成，允许数据的类型可以安全的序列化和反序列化。


效率：围绕sqlite3 C库构建，rusqlite以最小的开销提供高效的数据库操作。


rusqlite非常适合只需要简单可靠的数据库解决方案，而不需要复杂性的ORM框架的应用程序。
```rust
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

```



## Neon
Neon通过让PostgreSQL数据平台(是的，PostgreSQL不仅仅是一个数据库)使用兼容s3的存储作为后端，重新定义了数据库世界。虽然它使Postgres的单片架构变得复杂，但它也解决了许多问题：read-replicas现在使用单一的数据源而不是容易出错的复制，我们不再需要使用缓慢而昂贵的网络存储(如AWS的EBS)来获得高可用性的数据库，数据库的升级/降级现在只是生成一个新的容器/microVM的问题，不需要做任何复制。


## Datafusion
在过去的数据系统中，我们看到了不同层的分离：查询引擎、内存表示和存储。Datafusion是一个新的高性能和可扩展的查询引擎，它允许数据工程师使用Rust/Python的高级接口直接查询数据源，或者使用它作为查询层来构建数据系统，以构建和优化查询任务。它已经支持了许多令人印象深刻的项目，如InfluxDB、GreptimeDB和paradeDB。
![](../learning/src/objInfo/assets/Pasted%20image%2020241031214646.png)
```rust
use datafusion::prelude::*;  
use object_store::http::HttpBuilder;  
use std::sync::Arc;  
use url::Url;  
  
#[tokio::main]  
async fn main() -> Result<()> {  
    let ctx = SessionContext::new();  
  
    let base_url = Url::parse("https://github.com").unwrap();  
    let http_store = HttpBuilder::new()  
        .with_url(base_url.clone())  
        .build()  
        .unwrap();  
    ctx.register_object_store(&base_url, Arc::new(http_store));  
  
    ctx.register_csv(  
        "aggregate_test_100",  
        "https://github.com/apache/arrow-testing/raw/master/data/csv/aggregate_test_100.csv",  
        CsvReadOptions::new(),  
    )  
    .await?;  
  
    let df = ctx  
        .sql("SELECT c1,c2,c3 FROM aggregate_test_100 LIMIT 5")  
        .await?;  
  
    df.show().await?;  
  
    Ok(())  
}
```

## PGRX

正如我们在Neon中看到的，PostgresSQL不再是一个简单的数据库了。它已经成为一个“数据内核”，管理如何存储和查询数据，就像Linux是一个“计算内核”，管理进程和资源一样。因此，开发人员不满足于用c语言构建扩展是很自然的。有了pgrx，问题就解决了，我们现在可以用Rust构建快速、安全、可靠的Postgres扩展。