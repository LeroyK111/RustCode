/*
Remoc - 远程多路复用对象和通道。Remoc使Rust程序之间的远程交互无缝而流畅。通过单个底层传输，如TCP或TLS。它提供：

支持多个通道，如：MPSC，oneshot，watch等。



提供远程同步原语



可以调用远程对象(RPC)上的远程函数和trait方法



可以远程可观察集合



Remoc是100%用Rust编写的，建立在Tokio之上，可以使用Serde支持的任何类型和数据格式，并且不依赖于任何特定的传输类型。

Rust程序中的一个常见模式是使用通道在线程和异步任务之间进行通信。通道的设置在一行中完成，它在很大程度上避免了共享状态及相关的复杂性。Remoc通过提供在远程连接上无缝工作的通道，将这种编程模型扩展到分布式系统。

为此，它使用Serde在数据通过底层(可能是TCP网络连接、WebSocket、UNIX管道，甚至是串行链接)传输时对数据进行序列化和反序列化。

打开新通道很简单，只需将新通道的一半发送到现有通道上，就像在本地线程和任务之间所做的那样。所有通道都在同一个远程连接上进行多路复用，数据以块的形式传输，以避免在传输大消息时一个通道阻塞另一个通道。

基于远程通道，Remoc允许调用远程函数和闭包。此外，可以使用自动生成的客户端和服务器实现使trait成为可远程调用的，类似于经典的远程过程调用(RPC)模型。

Remoc的大部分功能都是由crate特性控制的。以下功能是可用的：

serde：启用编解码器模块，并为所有配置和错误类型实现序列化和反序列化。



rch：启用由RCH模块提供的远程通道。



rfn：支持由RFN模块提供的远程函数调用。



robj：启用由Robj模块提供的远程对象实用程序。



robs：支持由Robs模块提供的远程可观察集合。



rtc：支持由RTC模块提供的远程trait调用。



默认情况下，所有功能都是启用的，并且默认使用JSON编解码器。
*/

use remoc::prelude::*;
use std::net::Ipv4Addr;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    futures::join!(connect_client(), connect_server());
}

// 通过TCP建立到服务器的远程连接
async fn connect_client() {
    // 等待服务器启动
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // 建立TCP连接
    let socket = TcpStream::connect((Ipv4Addr::LOCALHOST, 9870))
        .await
        .unwrap();
    let (socket_rx, socket_tx) = socket.into_split();

    // 连接总是双向的，但是我们可以丢掉不需要的接收器。
    let (conn, tx, _rx): (_, _, rch::base::Receiver<()>) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
            .await
            .unwrap();
    tokio::spawn(conn);

    // 运行客户端
    client(tx).await;
}

// 在服务器上运行，通过TCP接受来自客户端的远程连接。
async fn connect_server() {
    // 监听传入的TCP连接
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 9870))
        .await
        .unwrap();
    let (socket, _) = listener.accept().await.unwrap();
    let (socket_rx, socket_tx) = socket.into_split();

    // 通过TCP建立远程连接
    let (conn, _tx, rx): (_, rch::base::Sender<()>, _) =
        remoc::Connect::io(remoc::Cfg::default(), socket_rx, socket_tx)
            .await
            .unwrap();
    tokio::spawn(conn);

    // 运行服务器
    server(rx).await;
}

// 用户定义的数据结构需要实现序列化和反序列化。
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CountReq {
    up_to: u32,
    // 大多数Remoc类型(如通道)都可以包含在可序列化的数据结构中，以便传输到远程端点.
    seq_tx: rch::mpsc::Sender<u32>,
}

// 它向服务器发送计数请求，并在通过新建立的MPSC通道进行计数时接收每个数字.
async fn client(mut tx: rch::base::Sender<CountReq>) {
    // 通过在现有的远程通道上发送seq_tx，将自动创建一个新的远程通道并将其连接到服务器。
    // 这一切都发生在现有的TCP连接中。
    let (seq_tx, mut seq_rx) = rch::mpsc::channel(1);
    tx.send(CountReq { up_to: 4, seq_tx }).await.unwrap();

    // 通过新通道接收已计数的数字。
    while let Some(i) = seq_rx.recv().await.unwrap() {
        println!("接收到的数字：{i}");
    }
}

// 从客户端接收计数请求并发送每个数字，它是通过客户端提供的MPSC通道发送器进行计数的.
async fn server(mut rx: rch::base::Receiver<CountReq>) {
    // 接收计数请求和用于计数的通道发送方
    while let Some(CountReq { up_to, seq_tx }) = rx.recv().await.unwrap() {
        for i in 0..up_to {
            // 通过提供的通道发送每个计数
            seq_tx.send(i).await.unwrap();
        }
    }
}