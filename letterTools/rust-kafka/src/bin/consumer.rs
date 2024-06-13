use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

async fn consume_files_from_kafka(
    topic: &str,
    brokers: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Kafka消费者配置
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "file-consumer-group")
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .create()?;

    // 订阅Kafka主题
    consumer.subscribe(&[topic])?;

    // 存放目录
    let output_directory = "output/";

    // 开始消费消息
    loop {
        match consumer.recv().await {
            Ok(msg) => {
                let key = msg.key().map(|k| String::from_utf8_lossy(k));
                let payload = msg.payload().unwrap();

                if let Some(filename) = key {
                    // 创建文件路径
                    let file_path = format!("{}{}", output_directory, filename);

                    // 将有效负载写入文件
                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&file_path)
                        .await?;

                    file.write_all(payload).await?;
                    println!("File saved: {}", file_path);
                } else {
                    println!("Message without a filename key");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Message without a filename key",
                    )));
                }
            }
            Err(e) => {
                eprintln!("Error consuming message: {:?}", e);
                return Err(Box::new(e));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "file-sharing-topic";
    let brokers = "localhost:9092";

    consume_files_from_kafka(topic, brokers).await?;

    Ok(())
}