# Kameo: 基于 Tokio 构建的异步 Actor

使用宏的方式（也有不用宏的方式）定义一个  actor

```rust
// Define the actor state
#[derive(Actor)]
struct Counter {
    count: i64,
}

// Define messages
#[actor]
impl Counter {
    #[message]
    fn inc(&mut self, amount: u32) -> Result<i64, Infallible> {
        self.count += amount as i64;
        Ok(self.count)
    }
}
```
执行方式

```rust
use kameo::{Spawn, ActorRef};

let counter_ref: ActorRef<Counter> = Counter { count: 0 }.spawn();

let count = counter_ref.send(Inc(42)).await?;
println!("Count is {count}");
```

# Rust Tokio取消任务的几种模式

Rust提供了对异步编程的支持，它可以生成异步任务，然后通过运行时执行器在操作系统线程之间调度执行。

与Rust中的所有东西一样，异步编程必须是内存安全的，因此需要确保借用检查器可以编译通过。

这篇文章是关于任务取消模式的，下面我们来介绍Tokio任务的取消模式。


Select 和 Channels

所有这些模式的核心是两个tokio特性：

channel：用于任务间通信



select：用于等待多个异步计算(不一定是任务!)



Tokio channel看起来有点复杂，但同时就程序的内存安全和弹性而言，它很强大。Tokio channel创建了两个不同的对象，用于任务之间的通信，不能同时使用一个通道对象来接收和发送。

Tokio提供的频道实际上有四种：

mpsc：多个生产者，单一消费者



oneshot：用于发送和接收单个值，发送后，通道关闭。



broadcast：多个发送者，多个消费者



watch：单一生产者，多个消费者



Drop JoinHandle不会取消任务

JoinHandle在删除关联的任务时将其分离，这意味着不再有任何任务句柄，也没有办法对其进行连接。

每次在tokio中生成任务时，都会返回JoinHandle。可以使用join句柄来等待任务完成，但是认为可以使用它来简单地通过删除任务来强制终止任务是错误的。这里有一个愚蠢的例子：

```rust
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do some work
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("Task completed");
    });

    // 100毫秒后取消任务
    time::sleep(Duration::from_millis(100)).await;
    drop(handle);

    println!("Task was cancelled");
}
```

丢弃句柄并不会取消正在运行的任务！


## Abort任务

这是取消任务的最极端的方式，没有清理的空间：

```rust
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do some work
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Task completed");
    });

    // 100毫秒后取消任务
    time::sleep(Duration::from_millis(100)).await;
    handle.abort();
    time::sleep(Duration::from_secs(2)).await;

    println!("Task was cancelled");
}
```

## 使用oneshot channel

oneshot channel允许通道上的发送单个值，可以由多个接收器侦听。与drop模式不同，此模式允许通道执行一些清理工作。这里有一个例子：
```rust
use tokio::sync::oneshot;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    let task = tokio::spawn(async move {
        tokio::select! {
            _ = rx => {
                println!("Task is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task completed normally");
            }
        }
        println!("Task is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send(());

    // 等待任务完成
    let _ = task.await;
}
```

运行结果如下：
Task is cancelling...
Task is cleaning up

oneshot channel的限制是你不能用它来取消多个任务。

## 使用broadcast channel取消多个任务

如果要取消多个任务，可以使用broad channel。可以有多个生产者向通道发送信息，也可以有多个消费者从通道接收信息。每个接收方都可以看到在通道上发送的每个值。

这里有一个简单的例子，来演示如何使用它来取消多个任务：
```rust
use tokio::sync::broadcast;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(1);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        tokio::select! {
            _ = rx1.recv() => {
                println!("Task 1 is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 1 completed normally");
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        tokio::select! {
            _ = rx2.recv() => {
                println!("Task 2 is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 2 completed normally");
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send(());

    // 等待任务完成
    let _ = tokio::join!(task1, task2);
}

```

运行结果如下：
Task 2 is cancelling...
Task 2 is cleaning up
Task 1 is cancelling...
Task 1 is cleaning up
取消的顺序可能会有所不同，因为任务可能会以不同的顺序取消！

如果只想从单个任务向多个任务发送取消信号，那么broad channel可能有点过度，因为它提供了在多个任务之间传递消息的所有机制。

如果既需要消息传递又需要消息取消，这很方便。但如果只需要消息取消，还有更好的方法，开销更少：watch channel。

## 使用watch channel取消多个任务

watch channel是多个消费者频道的单一生产者。watch channel给了任务清理自己的机会。缺点是，消费者只能看到通道上发送的最近的值——这意味着如果任务在通道上发送了一个值之后启动，它可能会错过它，因此不会被取消，所以要小心这一点。这里有一个简单的例子：

```rust
use tokio::sync::watch;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = watch::channel(false);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx1.changed() => {
                    if *rx1.borrow() {
                        println!("Task 1 is cancelling...");
                        break;
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 1 completed normally");
                    break;
                }
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx2.changed() => {
                    if *rx2.borrow() {
                        println!("Task 2 is cancelling...");
                        break;
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    let _ = tx.send(true);

    // 等待任务完成
    let _ = tokio::join!(task1, task2);
}
```


## 取消令牌

官方的Tokio文档中列出了一种名为CancellationToken的东西，用于优雅关机。这在tokio crate本身中不可用，但在相关的toko_util crate中可用。
```rust
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Create a CancellationToken
    let token = CancellationToken::new();

    let token1 = token.clone();
    let token2 = token.clone();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token1.cancelled() => {
                        println!("Task 1 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 1 completed normally");
                    break;
                }
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token2.cancelled() => {
                        println!("Task 2 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    sleep(Duration::from_millis(100)).await;

    // 发送取消信号
    token.cancel();

    // 等待任务完成
    let _ = tokio::join!(task1, task2);
}
```

请注意我们是如何克隆令牌的，以便将其移动到各个异步任务中。