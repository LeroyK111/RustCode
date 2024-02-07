use tokio_stream::{once, StreamExt};

#[tokio::main]
async fn main() {
    /*
    Streams(流)
    Tokio中的streams是异步值的序列，它们类似于迭代器，但不是阻塞执行。streams可以表示事件序列或异步I/O。例如，streams可以表示从WebSocket连接传入的消息。
    */
    let mut stream = once(5);

    while let Some(v) = stream.next().await {
        println!("{}", v);
    }
}


use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    /*
    Channels(通道)

Tokio中的channel用于异步任务之间的通信，可以使用它们将数据从一个任务发送到另一个任务。channel可以是有界的，也可以是无界的，有界channel对一次可以容纳多少消息有限制，而无界channel则没有这样的限制。
    */
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                break;
            }
        }
    });

    while let Some(v) = rx.recv().await {
        println!("{}", v);
    }
}


use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    /*
    
Timeouts(超时)

Tokio提供了设置任务超时的实用程序，可以使用超时来防止任务花费太长时间。如果一个任务在超时前没有完成，它将被取消。
    */
    let result = timeout(Duration::from_secs(5), sleep(Duration::from_secs(10))).await;

    match result {
        Ok(_) => println!("Task finished in time"),
        Err(_) => println!("Task timed out"),
    }
}


use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    错误处理

Tokio中的错误处理方式与同步Rust中的相同，也使用Result和Option类型处理可能失败的函数。当使用?操作符时，如果表达式为Err，则函数将立即返回并将控制权交还给执行器。
    */
    let mut file = File::open("foo.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("{}", contents);
    Ok(())
}