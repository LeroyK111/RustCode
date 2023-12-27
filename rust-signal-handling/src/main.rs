/*
! 信号中断
*/
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化信号
    let mut sigint = signal(SignalKind::interrupt())?;

    // 非阻塞
    match sigint.recv().await {
        // 一旦该方法被调用，你就可以监听该信号并使用.recv()方法处理它，如下所示：
        Some(()) => println!("Received SIGINT signal"),
        None => eprintln!("Stream terminated before receiving SIGINT signal"),
    }

    for num in 0..10000 {
        println!("{}", num)
    }

    Ok(())
}