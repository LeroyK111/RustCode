use wtransport::ClientConfig;
use wtransport::Endpoint;

#[tokio::main]
async fn main() {
    // 配置 client 端
    let config = ClientConfig::builder()
        .with_bind_default()
        .with_no_cert_validation()
        .build();

    // 连接服务器的4433端口
    let connection = Endpoint::client(config)
        .unwrap()
        .connect("https://[::1]:4433")
        .await
        .unwrap();

    // 使用双向通信流发送消息
    let mut stream = connection.open_bi().await.unwrap().await.unwrap();
    stream.0.write_all(b"HELLO").await.unwrap();
    stream.0.finish().await.unwrap();

    // 使用单向通信流发消息
    let mut stream = connection.open_uni().await.unwrap().await.unwrap();
    stream.write_all(b"WORLD").await.unwrap();
    stream.finish().await.unwrap();

    // 发送数据报消息
    connection.send_datagram(b"Hello, world!").unwrap();

    tokio::time::sleep(Duration::from_secs(3)).await;
}