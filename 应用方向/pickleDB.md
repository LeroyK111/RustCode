# Rust 版本的 PickleDB
PickleDB 是一个用 Rust 编写的轻量级且简单的键值存储，很大程度上受到 Python PickleDB 的启发。PickleDB 有趣且易于使用

```rust
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

fn main() {

    // create a new DB with AutoDump (meaning every change is written to the file)
    // and with Json serialization (meaning DB will be dumped to file as a Json object)
    let mut db = PickleDb::new("example.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);

    // set the value 100 to the key 'key1'
    db.set("key1", &100).unwrap();

    // print the value of key1
    println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

    // load the DB from the same file
    let db2 = PickleDb::load("example.db", PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).unwrap();

    // print the value of key1
    println!("The value of key1 as loaded from file is: {}", db2.get::<i32>("key1").unwrap());
}
```

```toml
[dependencies]
pickledb = "0.5.1"
```