/*
在src/bin目录下，创建一个ws_cli.rs文件。在文件中定义websocket_client函数，建立WebSocket连接并管理消息：
*/

use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

async fn websocket_client(topic_url: &str) {
    // 解析要连接WebSocket服务器的URL
    let url = Url::parse(topic_url).expect("Invalid URL");

    // 连接到WebSocket服务器
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("WebSocket client connected");

    let (mut write, mut read) = ws_stream.split();
    let message = Arc::new(RwLock::new(String::new()));
    let message_1 = message.clone();
    // 生成一个任务来处理传入的消息
    tokio::spawn(async move {
        let msg_lock = message_1.clone();
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    let mut ms = msg_lock.write().await;
                    *ms = msg.to_text().unwrap().to_string();
                    println!("Received message: {}", msg.to_text().unwrap());
                }
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }
    });

    // 发送消息
    loop {
        let msg_lock = message.clone();
        let ms = msg_lock.read().await;
        if let Err(e) = write.send(Message::Text(ms.to_string())).await {
            eprintln!("Error sending message: {:?}", e);
            break;
        }
        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    websocket_client("ws://127.0.0.1:3030/subscribe/newtopic").await;
}