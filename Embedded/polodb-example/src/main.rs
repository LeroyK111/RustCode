/*
PoloDB是一个用Rust编写的嵌入式数据库，如果你正在用Rust开发应用程序，PoloDB将非常适合你，这是因为PoloDB实现了轻量级的MongoDB，本质上与MongoDB兼容。
*/

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

// 插入数据库
fn insert_one(db: &Database) -> InsertOneResult {
    let collection = db.collection("books");
    collection.insert_one(Book {
        title: "The Three-Body Problem".to_string(),
        author: "Liu Cixin".to_string(),
    }).unwrap()
}

// 创建数据库
fn create_db(path: &str) -> Database {
    if path.is_empty() {
        Database::open_memory().unwrap()
    } else {
        Database::open_file("test-polo.db").unwrap()
    }
}

// 查询数据
fn find(db: &Database) {
    let collection = db.collection::<Book>("books");

    let book = collection.find_one(doc! {"author": "Liu Cixin"}).unwrap();
    println!("name: {:?}", book);

    let books = collection.find(None).unwrap();
    for book in books {
        println!("name: {:?}", book);
    }
}

// 修改数据
fn update(db: &Database) {
    let collection = db.collection::<Book>("books");
    collection.update_one(doc! {
        "author": "Liu Cixin"
    }, doc! {
        "$set": doc! {
            "title": "The Three-Body Problem 1",
        },
    }).unwrap();

    collection.update_many(doc! {
        "author": "George Orwell"
    }, doc! {
        "$set": doc! {
            "title": "xxxxx",
        },
    }).unwrap();
}

// 删除数据
fn delete(db: &Database) {
    let collection = db.collection::<Book>("books");
    collection.delete_one(doc! {
        "author": "Liu Cixin"
    }).unwrap();

    collection.delete_many(doc! {
        "author": "George Orwell"
    }).unwrap();
}



fn main() {
    println!("Hello, world!");
}
