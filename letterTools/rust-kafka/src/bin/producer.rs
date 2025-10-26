use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout::Never;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::time::Instant;

async fn produce_file_to_kafka(
    file_path: &str,
    topic: &str,
    brokers: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 配置Kafka生产者
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.max.bytes", (1 * 1024 * 1024).to_string())
        .create()?;

    // 读取文件块，并产生每个块作为一个带有Kafka头的消息
    let mut file = File::open(file_path).await?;
    let mut reader = BufReader::new(&mut file);
    let mut buffer = vec![0u8; 1000 * 1024]; // 1000KB缓冲区大小

    let mut total_bytes_read = 0;
    let mut total_chunks_sent = 0;

    let filename_with_ext = match file_path.rsplit_once('/') {
        Some((_, name)) => name,
        None => file_path,
    };

    let now = Instant::now(); // 起始时间

    // 开始读取文件
    loop {
        let n = reader.read(&mut buffer).await.unwrap_or(0);
        if n == 0 {
            break;
        }

        total_bytes_read += n;
        total_chunks_sent += 1;

        // 生成文件块作为带有Kafka头的消息
        let record = FutureRecord::to(topic)
            .payload(&buffer[0..n]) // 只发送实际读取的数据
            .key(filename_with_ext);

        match producer.send(record, Never).await {
            Ok(_) => println!("File chunk {} sent successfully.", total_chunks_sent),
            Err((kafka_error, _)) => {
                eprintln!("Error sending remaining data: {:?}", kafka_error);
                return Err(Box::new(kafka_error));
            }
        }

        // 为下一次迭代重置缓冲区
        buffer = vec![0u8; 1000 * 1024];
    }
    // 处理剩余数据(如有)
    if total_bytes_read % (1000 * 1024) != 0 {
        let remaining_data = &buffer[0..(total_bytes_read % (1000 * 1024))];

        // 将剩余的数据生成作为带有Kafka头的消息
        let record = FutureRecord::to(topic)
            .payload(remaining_data)
            .key(filename_with_ext)
            .headers(OwnedHeaders::new().insert(Header {
                key: "file_name",
                value: Some(filename_with_ext),
            }));

        match producer.send(record, Never).await {
            Ok(_) => println!("Remaining data sent successfully."),
            Err((kafka_error, _)) => {
                eprintln!("Error sending remaining data: {:?}", kafka_error);
                return Err(Box::new(kafka_error));
            }
        }
    }

    let elapsed_time = now.elapsed();
    println!("Running  took {} seconds.", elapsed_time.as_secs());
    println!("Total bytes read: {} bytes", total_bytes_read);

    // 关闭文件和producer
    drop(file);
    drop(producer);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./test.txt"; // 你的文件路径
    let topic = "file-sharing-topic";
    let brokers = "localhost:9092";

    produce_file_to_kafka(file_path, topic, brokers).await?;

    println!("File successfully sent to Kafka!");

    Ok(())
}