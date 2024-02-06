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

# Rust中的并发性：非阻塞与阻塞数据结构

非阻塞数据结构是一种并发数据结构，允许线程在不需要锁的情况下访问和修改它。这个特性减少了线程争用和死锁的可能性，从而提高了多线程应用程序的性能。


## 非阻塞数据结构的优点：

性能效率：与阻塞数据结构相比，非阻塞数据结构可以提供更高的吞吐量和更好的可伸缩性，特别是在高度并发的环境中。


避免死锁：由于它们不使用传统的锁，因此它们避免了死锁之类的问题，而死锁可能很难检测和解决。


容错：在某些情况下，它们更健壮，例如当持有锁的线程失败或在阻塞结构中挂起时，这可能导致整个系统停滞。




### 用例和应用程序：

实时系统：在响应时间至关重要的系统中，如交易系统或游戏服务器，非阻塞结构可以确保及时处理数据。


高性能计算：它们在需要并发数据访问和操作的科学计算和模拟中非常有用。


Web服务器和数据库：处理大量并发请求的服务器可以从非阻塞数据结构的效率中获益。


非阻塞数据结构通常使用原子操作，如比较-交换(CAS)，以确保多个线程可以安全地并发地修改数据。这些原子操作保证操作要么完全成功，要么无效，从而维护数据完整性。


## Rust中的非阻塞数据结构

Rust非常关注安全性和并发性，通过其标准库和外部crate为非阻塞数据结构提供了出色的支持。

标准库：Rust的标准库包括原子类型，如AtomicBool、AtomicIsize、AtomicUsize等，它们可用于构建非阻塞数据结构。


外部板crate：像crossbeam这样的crate提供了一系列非阻塞数据结构，如队列、双端队列和栈。


我们通常会使用原子操作，并通过Rust的所有权和借用规则仔细管理内存，确保不会发生数据竞争。
```rust
use std::sync::atomic::{AtomicUsize, Ordering};  
  
let counter = AtomicUsize::new(0);  
  
// Incrementing the counter safely in a concurrent environment  
counter.fetch_add(1, Ordering::Relaxed);
```

在这个例子中，fetch_add是一个原子操作，即使被多个线程并发访问，也可以安全地增加计数器。

Rust严格的并发性和安全性，保证其成为实现和使用非阻塞数据结构的理想语言，从而确保并发应用程序的性能和安全性。


## 比较非阻塞和阻塞数据结构

我们将探讨在并发Rust程序中实现同步的两种不同方法：非阻塞和阻塞。我们通过两个函数using_non_blocking和using_blocking实现从多个线程中增加共享计数器。

### 非阻塞方法

在using_non_blocking中，我们使用AtomicUsize和Arc(原子引用计数)来进行线程安全的操作和数据共享。AtomicUsize是一种支持无锁原子操作的原子变量。我们生成100,000个线程，每个线程将计数器精确地增加一次。fetch_add函数会自动增加计数器，确保线程安全，而不需要互斥锁。

```rust
use std::{  
    sync::{  
        atomic::{AtomicUsize, Ordering},  
        Arc, Mutex,  
    },  
    thread,  
};  
  
fn using_non_blocking() {  
    println!("Entering function: using_non_blocking");  
  
    let counter = Arc::new(AtomicUsize::new(0));  
    let mut handles = vec![];  
  
    let start = std::time::Instant::now();  
  
    for _ in 0..100000 {  
        let counter_clone = Arc::clone(&counter);  
        let handle = thread::spawn(move || {  
            counter_clone.fetch_add(1, Ordering::Relaxed);  
        });  
        handles.push(handle);  
    }  
  
    for handle in handles {  
        handle.join().unwrap();  
    }  
  
    println!("Final value: {}", counter.load(Ordering::SeqCst));  
    println!(  
        "Exiting function: using_non_blocking (took {} ms)",  
        start.elapsed().as_millis()  
    );  
}
```

### 阻塞方法

在using_blocking中，我们使用互斥锁来保护计数器。这是阻塞方法的一个经典示例，其中每个线程必须在增加计数器之前获得锁。互斥锁确保对计数器的独占访问，防止数据竞争，但可能导致线程争用。

```rust
fn using_blocking() {  
    println!("Entering function: using_blocking");  
  
    let start = std::time::Instant::now();  
  
    let counter = Arc::new(Mutex::new(0));  
    let mut handles = vec![];  
  
    for _ in 0..100000 {  
        let counter_clone = Arc::clone(&counter);  
        let handle = thread::spawn(move || {  
            let mut num = counter_clone.lock().unwrap();  
            *num += 1;  
        });  
        handles.push(handle);  
    }  
  
    for handle in handles {  
        handle.join().unwrap();  
    }  
  
    println!("Final value: {}", counter.lock().unwrap());  
    println!(  
        "Exiting function: using_blocking (took {} ms)",  
        start.elapsed().as_millis()  
    );  
}


```

分析

最后输出结果：
```sh
Entering function: using_non_blocking
Final value: 100000
Exiting function: using_non_blocking (took 34738 ms)

Entering function: using_blocking
Final value: 100000
Exiting function: using_blocking (took 38158 ms)
```

在这种情况下，非阻塞方法稍微快一些。这可以归因于不必像阻塞方法那样获取和释放锁，从而减少了开销。然而，性能上的差异并不像人们想象的那么显著。这可能是由于几个因素造成的，例如:

生成和管理大量线程的开销。


操作本身的性质(一个简单的增量)，这可能不会完全暴露阻塞方法中潜在的争用问题。


现代CPU和编译器优化，可以减轻与锁相关的一些开销。


## 总结

这个比较说明了非阻塞和阻塞方法之间的选择取决于特定的用例、争用级别和所执行操作的性质。虽然非阻塞结构可以在高争用场景中提供性能优势，但在低争用或不太复杂的操作中，阻塞结构的简单性和有效性也不容忽视。

在Rust中，这两种方法都受益于该语言对安全性的加强，确保无论选择哪种方式，数据竞争和并发性问题都能得到有效管理，为并发编程提供了坚实的基础。
