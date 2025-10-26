/*
分析包括分析代码的运行时行为，以识别消耗大量时间或资源的区域。Rust有几个可用的分析工具，比如perf、Valgrind和flamgraph。当我们讨论内联函数时，我们会更多地使用Valgrind，但现在，让我们使用一下flamgraph。flamgraph是一个流行的Rust分析工具，它可以生成程序运行时堆栈跟踪的可视化图形。这些图被称为火焰图，提供了代码中时间花费的可视化表示，使其更容易查明性能瓶颈。要开始使用flamgraph，首先通过以下方式安装：
*/

// cargo install flamegraph

// 然后，你可以使用“cargo flamgraph”命令来测试编译好的二进制文件。

/*
如图所示，火焰图可视化地表示了代码的不同部分所花费的时间。每个框代表一个堆栈帧或一个函数调用。高度表示堆栈深度，最近的堆栈帧位于顶部，较旧的堆栈帧位于底部。子框架驻留在调用它们的函数之上。帧的宽度表示一个函数或它的子函数被处理的总时间。您可以将鼠标悬停在一个框架上以获取更多细节，并单击一个框架展开它以获得更细粒度的视图。每个帧的颜色并不重要，并且是随机的，除非你使用 --deterministic参数，这将在运行期间保持功能/颜色的一致性。
*/

//基准测试用于测量代码的性能，以比较不同的实现。Rust提供了一个名为Criterion的内置基准测试框架。要使用Criterion，将它作为一个依赖项添加到Cargo.toml中：

/*
[dev-dependencies]
criterion = { version = "0.5.3", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
*/

// 然后，你可以在benches/my_benchmark.rs中编写你的基准测试代码：
/*
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
*/

// 最后，用以下命令运行这个基准测试： cargo bench
/*

在进行优化时，请始终记住对代码进行基准测试，以确保所做的更改确实提高了程序的性能。如果基准测试没有显示出足够显著的速度改进，那么可能就不值得进行优化。
*/