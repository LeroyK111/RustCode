# FuturesUnordered 以及 future的顺序

FuturesUnordered 是 Rust 中用于并发子任务的强大工具，但如果使用不当，会导致死锁。

## 两种常见的 FuturesUnordered 使用模式：
缓冲流：将工作流表示为 futures 的流，并使用类似 buffered 适配器将其缓冲。
范围任务：将工作表示为“在” FuturesUnordered 上“生成”的任务，使用类似 moro 库的 API。
死锁的条件：
互斥：每个任务都需要独占资源。
等待资源：任务在等待其他资源时持有已分配的资源。
不可抢占：资源不能从持有者手中强制移除。
循环等待：存在任务链，每个任务都请求链中下一个任务持有的资源。

## 示例死锁：
使用异步生成器和互斥锁。
```rust

async gen {
    let mut guard = mutex.lock().await;
    yield x;
    yield y;
    drop(guard);
};

for await elem in iter {
    let mut guard = mutex.lock().await;
    println!("{}", elem);
    drop(guard);
}
```
## 预防死锁：
通过显式声明并发性来避免控制流中的隐式共享资源。
优先使用范围任务而不是缓冲流，因为它们使资源依赖关系更清晰。
使用通道时要注意队列大小和循环依赖关系。
使用未缓冲通道测试以捕获潜在死锁。

## Rust 的优势：
所有权和借用可以防止死锁和复杂的同步。
范围任务应该集成到运行时中以获得更好的借用支持。
其他注意事项：
文章批评了使用大量任意值作为通道大小的做法，因为它隐藏了潜在的死锁。
它建议使用利用所有权和借用的模式。

## 总结
FuturesUnordered 是一个强大的工具，但需要谨慎使用以避免死锁。了解死锁的条件以及如何预防它们，并选择合适的模式来实现并发。Rust 的所有权和借用系统可以帮助防止死锁，并使代码更安全、更可靠。
