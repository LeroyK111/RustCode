# Rust 异步编程中常见的错误

## 1，忘记取消任务

让我们从 async 中所有错误的根源开始：任务取消。

```rust
async fn spawn_tasks() {
    let task_counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..100 {
        let task_counter = task_counter.clone();
        tokio::spawn(my_task(task_counter));
        // 这时发现问题了吧，有很多种可能，spawn_tasks永远不会返回。

// 问题在于，我们经常忘记Rust中的异步任务在完成之前不能保证正确运行。它们可以在任何等待点被取消或停止轮询。因此如果my_task是不正确的，那么task_counter可能永远不会递减。
        // tokio::spawn(tokio::time::timeout(Duration::from_millis(10), my_task(task_counter)));
    }

    while task_counter.load(Ordering::Relaxed) > 0 {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn my_task(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);

    let _ =  do_something().await;

    // 此指令可能永远不会运行，因为任务已被取消。
    // 或者发生了panic的事情
    task_counter.fetch_sub(1, Ordering::Relaxed);
}
```

修改为my_task

```rust
// 解决方案：当任务被drop时，使用钩子
async fn my_task(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);
    let _guard = scopeguard::guard((), |_| {
        task_counter.fetch_sub(1, Ordering::Relaxed);
    });

    let _ =  do_something().await;
}
// 通过使用scopeguard crate，我们可以注册一些代码块，以便在_guard被drop时运行。如果你喜欢宏，你甚至可以使用像Go语言的defer!语法：
async fn my_task(task_counter: Arc<AtomicUsize>) {
    task_counter.fetch_add(1, Ordering::Relaxed);
    defer! {
        task_counter.fetch_sub(1, Ordering::Relaxed);
    }

    let _ =  do_something().await;
}

```

## 2，Select和取消任务
在代码审查期间很容易忽略任务的取消问题，因为我们习惯于从上到下阅读代码，并将返回点视为函数中唯一的退出流。

任务取消问题经常在代码重构为select!时被发现。
```rust
async fn my_task(mut should_abort: tokio::sync::oneshot:Sender<()>) {
    loop {
        select! {
            biased;

            _ = should_abort.closed() => {
                println!("Aborting task");
                return;
            }
            _ = process.recv_msg() => {
                println!("Task completed");
            }
        }
    }
}
```
上面的代码是有效的，但是执行了不必要的额外操作。当我们使用select!宏，它评估所有分支，为每个分支创建一个全新的Future，当它退出select!，那些Future都丢掉了！

事实证明，当我们在循环中使用select!时，每次都会创建一个用于Future创建和销毁的循环，而这应该是不必要的。我们不需要在每次迭代时创建一个新的future来判断是否应该abort。

请记住，并非所有Future都可以安全取消。比如，无论你做什么，这个Future都是不可取消的，Drop它意味着你的应用程序将丢失缓冲的消息。

```rust
async fn read_exact_3() -> Vec<()> {
  // 当取消时，所有缓冲的msg都将丢失
  let mut buffer = Vec::with_capacity(3);

  while buffer.len() < 3 {
     let msg = read_one().await;
     buffer.push_back(msg);
  }

  buffer
}
```

我们通常想要的是在循环的每次迭代中重用我们的Future，以避免在循环的每次迭代中重新创建一个新的，这可能需要昂贵的初始化和drop代码。

解决方案：使用fuse和pin_mut将你的Future提升到循环之上

```rust
async fn my_task(mut should_abort: tokio::sync::oneshot:Sender<()>) {

        let should_abort = should_abort.closed().fuse();
        pin_mut!(should_abort);

        loop {
        select! {
            biased;

            _ = should_abort => {
                println!("Aborting task");
                return;
            }

            _ = process.recv_msg() => {
                println!("Processing message");
            }
        }
    }
}
```

代码现在不那么简单了，需要引入futures crate中的fuse和pin_mut!，在循环的每次迭代中重用should_abort future。

现在当你看到一个循环和一个select!问自己这两个问题：

1，我的Future能安全的取消吗?

2，难道我不能把我的Future提升到循环之外，以避免每次都创造和Drop它吗？

## 3，没有使用同步互斥锁

当你开始在Rust中使用Async时，你告诉自己，我现在在async/await的世界里，所以我必须使用异步互斥锁。

```rust
async fn my_task(mut should_abort: tokio::sync::oneshot::Sender<()>) {

    let workers = Arc::new(tokio::sync::Mutex::new(HashMap::new()));

    for i in 1..10 {
       let workers = workers.clone();
        tokio::spawn(async move {
            let mut workers = workers.lock().await;
            workers.insert(i, i);
        });
    }
}
```
事实证明，这是不必要的，而且性能不如使用常规同步互斥锁，因为它需要与任务执行器来回切换。

解决方案：使用常规同步互斥锁

上面的代码可以用普通的互斥锁重写：

```rust
async fn my_task(mut should_abort: tokio::sync::oneshot::Sender<()>) {
    let workers = Arc::new(std::sync::Mutex::new(HashMap::new()));

    for i in 1..10 {
       let workers = workers.clone();
        tokio::spawn(async move {
            let mut workers = workers.lock().unwrap();
            workers.insert(i, i);
        });
    }
}

```
代码是有效的，获取锁不再需要等待，并且比异步版本性能更好。

有时，你的锁临界区有.await，你认为需要使用异步互斥锁。但是，通常可以使用同步版本来避免这个.await。例如，如果在无界通道中推送msg，可以使用try_send，它永远不会返回ErrorFull，并且它允许不使用.await。许多异步原语在同步代码中都有等价的使用。
