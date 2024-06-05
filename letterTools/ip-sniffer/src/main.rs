/*
! 使用Rust构建IP嗅探器

?使用了 tokio 异步运行时库
?使用bpaf库处理命令行参数。

Bpaf用于解析命令行参数

Std::io和Std::net用于输入/输出和网络操作。

Std::sync::mpsc用于在线程之间传递消息。

Tokio用于异步编程

*/

use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;


const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

/*
常量(MAX和IPFALLBACK)：这些是作为默认值使用的预定义值。MAX设置结束端口的最大值，确保它不超过最大允许的端口号(65535)。IPFALLBACK提供了一个默认的IP地址(127.0.0.1，即本地主机)，以防用户没有指定。

Arguments struct：该结构体定义了程序将接受的命令行参数的类型和约束。

address：该字段接受IP地址作为输入。如果用户不提供地址，则默认为IPFALLBACK。

start_port和end_port：这些字段定义要扫描的端口范围。start_port必须大于0，end_port必须小于等于65535。两者都提供了默认值，start_port从1开始，end_port使用可能的最大端口号。

使用bpaf进行参数解析有助于创建健壮的命令行界面、对用户友好，并且不易出错，因为它可以优雅地对参数进行验证和使用默认值。
*/

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

// 端口扫描
/*
定义了一个名为scan的异步函数，它接受三个参数：

tx：sender类型的channel发送方，用于将数据(在本例中为端口号)发送到程序的另一部分。
start_port：要检查的端口号。
addr：检查端口的IP地址。

处理Connection Result：
Ok(_)：如果连接成功(表示端口打开)，打印一个点(.)到标准输出，作为连接成功的视觉指示。io::stdout().flush().unwrap()通过刷新标准输出缓冲区，确保点立即显示在屏幕上。tx.send(start_port).unwrap()通过tx通道发送打开的端口号，由程序的另一部分处理或记录。

Err(_)：如果连接失败(表示端口已关闭)，则什么也不发生。
通过检查指定范围内的每个端口以确定其是否打开，此功能对于执行端口扫描是必不可少的。它利用异步编程来有效地处理长时间运行的网络操作，而不会阻塞程序其他部分的执行。这允许同时扫描多个端口，大大加快了进程。

*/
async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        Err(_) => {}
    }
}


#[tokio::main]
async fn main() {
    // 配置
    let opts = arguments().run();
    // 信道队列
    let (tx, rx) = channel();

    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();
        task::spawn(async move { scan(tx, i, opts.address).await });
    }

    drop(tx);

    let mut out = vec![];
    for p in rx {
        out.push(p);
    }

    println!();

    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

/*
调用arguments()函数，该函数构造和解析命令行参数，返回存储在opts中的arguments结构体的实例。

创建了一个多生产者，单消费者(MPSC)通道。Tx是发送者，rx是接收者。此通道用于异步任务之间的通信。

循环遍历命令行参数中指定的从start_port到end_port的端口。

为每个端口生成一个新的异步任务，调用scan函数。每个任务将尝试连接到其分配的端口，并通过通道发送结果。

显式删除原始发送方。这很重要，因为它表示将不再在此通道上发送消息，接收方在处理所有发送的消息后退出其循环。

然后，对打开端口的向量进行排序打印。
*/

// cargo run -- --address 192.168.1.2 --start 1 --end 60000

