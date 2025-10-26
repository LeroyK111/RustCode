# 使用tcp-console解锁动态应用程序控制

在应用程序的生命周期中，经常需要能够与运行中的系统进行交互的工具——无论是验证运行时参数、检查系统运行状况，还是发出命令来动态调整其行为。

虽然跟踪框架擅长于为调试提供详细的可观察性，但它们并不总是满足实时的、双向的通信需求。

这就是tcp-console出现的地方——它提供了一个轻量级的、特别的解决方案，用于通过TCP接口监视和控制应用程序。

使用tcp-console，不仅可以获取关键信息，还可以动态地发送命令以更改应用程序行为，从而无需产生跟踪基础设施或外部监视工具的开销。

tcp-console并不打算取代跟踪工具。相反，它提供了一个轻量级的，几乎是特别的解决方案：

- 运行状况检查：快速获取基本操作指标。
- 运行时配置：动态修改特定参数以测试或调整行为。
- 监控：在没有专用工具的情况下检查应用程序状态。

对于较小的应用程序或在开发的早期阶段，tcp-console在不牺牲功能的情况下提供了简单的便利操作。

## tcp-console的主要特点

tcp-console可以通过基于tcp的接口与应用程序交互，能够实时发送命令或查询信息，主要特点包括：

1，命令多功能性

接受用于快速测试的纯文本命令和用于结构化交互的强类型命令。

2，易于设置

只需几行代码就可以启动一个监听本地主机的控制台，不需要跟踪跨度或外部监视设置。

3，大规模并发

它由Tokio提供异步运行时，可以有效地处理多个同时连接。

4，安全第一

默认为本地主机连接，在临时操作期间保证应用程序的安全。

## cargo.toml 配置
```toml
[dependencies]  
tcp-console = "0.1.1"  
async-trait = "0.1.83"  
bytes = { version = "1.9.0", features = ["serde"] }  
thiserror = "2.0.6"  
tokio = { version = "1.41.1", features = ["full"] }  
tokio-util = { version = "0.7.12", features = ["codec"] }  
futures-util = { version = "0.3.31", features = ["sink"] }  
tracing = "0.1.41"  
bcs = "0.1.6"  
serde = { version = "1.0.215", features = ["derive"] }  
anyhow = "1.0.93"  
tracing-subscriber = { version = "0.3.19", features = ["env-filter"] }
```


## src/main.rs

```rust
use async_trait::async_trait;  
use bytes::Bytes;  
use serde::{Deserialize, Serialize};  
use std::time::Duration;  
use tcp_console as console;  
use tcp_console::{Subscription, SubscriptionError};  
use tokio::{signal, time};  
use tracing::debug;  
use tracing_subscriber::EnvFilter;  
  
#[tokio::main]  
asyncfn main() -> anyhow::Result<()> {  
    init_tracing();  
  
    let port = 3838;  
  
    let console = console::Builder::new()  
        .port(port)  
        .welcome("Welcome to TCP console!")  
        .subscribe(Services::Logger, Logger)?  
        .subscribe(Services::Exec, Exec)?  
        .subscribe(  
            Services::Status,  
            Status {  
                connections: 11,  
                health: "Operational".to_string(),  
            },  
        )?  
        .accept_only_localhost()  
        .build()?;  
  
    console.spawn().await?;  
  
    tokio::spawn(asyncmove {  
        letmut client = console::Client::new(  
            format!("127.0.0.1:{port}")  
                .parse()  
                .expect("Failed to parse socket address"),  
        )  
        .await  
        .expect("Failed to create client");  
  
        client  
            .weak_send("status")  
            .await  
            .expect("Failed to send unknown message");  
  
        let status = client.weak_read().await.expect("Failed to read");  
        debug!("{status:?}");  
  
        time::sleep(Duration::from_secs(2)).await;  
  
        client  
            .send(Services::Logger, &"Typed LoggerMessage")  
            .await  
            .expect("Failed to send logger message");  
  
        time::sleep(Duration::from_secs(2)).await;  
  
        client  
            .send(Services::Exec, &"Typed ExecMessage")  
            .await  
            .expect("Failed to send exec message");  
  
        time::sleep(Duration::from_secs(2)).await;  
  
        client  
            .send(Services::Unknown, &"Typed UnknownMessage")  
            .await  
            .expect("Failed to send unknown message");  
    });  
  
    signal::ctrl_c().await?;  
  
    console.stop();  
  
    time::sleep(Duration::from_millis(100)).await;  
  
    Ok(())  
}  
  
fn init_tracing() {  
    tracing_subscriber::fmt()  
        .with_env_filter(EnvFilter::from_default_env())   
        .with_target(true)   
        .init();  
}  
  
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]  
enum Services {  
    Logger,  
    Exec,  
    Status,  
    Unknown,  
}  
  
struct Logger;  
  
#[async_trait]  
impl Subscription for Logger {  
    asyncfn handle(&self, message: Bytes) -> Result<Option<Bytes>, SubscriptionError> {  
        let message =  
            bcs::from_bytes::<String>(message.as_ref()).expect("Must deserialize message");  
        debug!("[Logger] request to process a strongly typed message: `{message}`");  
        Ok(None)  
    }  
  
    asyncfn weak_handle(&self, _message: &str) -> Result<Option<String>, SubscriptionError> {  
        Ok(None)  
    }  
}  
  
struct Exec;  
  
#[async_trait]  
impl Subscription for Exec {  
    asyncfn handle(&self, message: Bytes) -> Result<Option<Bytes>, SubscriptionError> {  
        let message =  
            bcs::from_bytes::<String>(message.as_ref()).expect("Must deserialize message");  
        debug!("[Exec] request to process a strongly typed message: `{message}`");  
        Ok(None)  
    }  
  
    asyncfn weak_handle(&self, _message: &str) -> Result<Option<String>, SubscriptionError> {  
        Ok(None)  
    }  
}  
  
// 表示某些系统状态的结构体  
#[derive(Debug)]  
#[allow(dead_code)]  
struct Status {  
    connections: u32,  
    health: String,  
}  
  
#[async_trait]  
impl Subscription for Status {  
    asyncfn handle(&self, _message: Bytes) -> Result<Option<Bytes>, SubscriptionError> {  
        debug!("[Status] request to process a strongly typed message");  
  
        Ok(None)  
    }  
  
    asyncfn weak_handle(&self, message: &str) -> Result<Option<String>, SubscriptionError> {  
        Ok(if message == "status" {  
            Some(format!("{self:#?}"))  
        } else {  
            None  
        })  
    }  
}
```

- Console在端口3838上监听，并使用欢迎消息向客户端表示欢迎。
- 它为特定命令注册自定义处理程序(Logger、Exec和Status)。
- Status处理程序根据文本命令状态提供系统运行状况信息。

要进行交互，可以使用netcat这样的工具：
```
nc localhost 3838
```
输入status将产生一个响应：
```shell
Status {`    `connections: 11,`    `health: "Operational",``}
```

## 为什么选择tcp-console？

tcp-console证明了轻量级工具仍然可以非常强大，它的目标不是取代健壮的跟踪或监视解决方案，而是在这些工具可能不必要或过于复杂的情况下弥补差距。

轻量级监控

非常适合在较小的应用程序或早期开发阶段验证运行状况或操作数据。

实时配置

允许运行时调整参数，简化实验和微调。

可扩展的设计

处理强类型命令和自定义实现，随您的需求而增长。