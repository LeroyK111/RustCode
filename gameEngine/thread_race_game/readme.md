# 使用Rayon在Rust中构建一个线程竞赛游戏

Rust中的并行性允许同时执行多个操作，充分利用多核处理器的优势。Rayon crate是一个功能强大的数据并行库，能够以最少的样板代码并行运行任务。

我们将构建一个游戏，其中多个线程竞相完成一个任务，用户可以打赌哪个线程将首先完成。这个游戏将帮助你了解如何在Rust中使用Rayon库管理和利用线程。

使用以下命令创建一个Rust新项目：
```sh
cargo new thread_race_game
```

