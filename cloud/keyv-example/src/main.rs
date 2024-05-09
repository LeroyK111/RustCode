use keyv::Keyv;


// 内存存储(默认)
// #[tokio::main]
// async fn main() {
//     // 用默认内存存储初始化' keyv '
//     let keyv = Keyv::default();

//     // 设置一个值
//     keyv.set("my_key", "my_value").await.unwrap();

//     // 获取值
//     if let Some(value) = keyv.get("my_key").await.unwrap() {
//         println!("Retrieved value: {}", value);
//     } else {
//         println!("Value not found");
//     }

//     // 删除值
//     keyv.remove("my_key").await.unwrap();
// }


// 与存储适配器一起使用
// docker run --name keyv-redis-test -p 6379:6379 -d redis:latest  
// 调整keyv以使用不同的存储适配器，如Redis。

use keyv::{adapter::redis::RedisStoreBuilder, Keyv};

#[tokio::main]
async fn main() {
    // 使用默认TTL初始化Redis存储
    let store = RedisStoreBuilder::new()
        .uri("redis://localhost:6379")
        .default_ttl(3600) // 1 hour TTL
        .build()
        .await
        .unwrap();

    // 用Redis store创建keyv实例
    let keyv = Keyv::try_new(store).await.unwrap();

    // 设置值和检索值
    keyv.set("my_key", "my_value").await.unwrap();
    let value = keyv
        .get("my_key")
        .await
        .unwrap()
        .expect("Key not found");

    println!("Retrieved value: {}", value);
}


