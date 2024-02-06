/*
在本文中，我们介绍了await- tree作为Async Rust中可观察性的强大工具。await- tree是为Async Rust原生设计的回溯工具，它允许开发者实时观察每个异步任务的执行状态，并分析不同future或任务之间的依赖阻塞关系。
*/
use std::time::Duration;

use await_tree::{Config, InstrumentAwait, Registry};
use futures::future::{join, pending};
use tokio::time::sleep;

async fn bar(i: i32) {
    // `&'static str` span
    baz(i).instrument_await("baz in bar").await
}

async fn baz(i: i32) {
    // runtime `String` span is also supported
    pending()
        .instrument_await(format!("pending in baz {i}"))
        .await
}

async fn foo() {
    // spans of joined futures will be siblings in the tree
    join(
        bar(3).instrument_await("bar"),
        baz(2).instrument_await("baz"),
    )
    .await;
}

#[tokio::main]
async fn main() {
    let mut registry = Registry::new(Config::default());
    let root = registry.register((), "foo");
    tokio::spawn(root.instrument(foo()));

    sleep(Duration::from_secs(1)).await;
    let tree = registry.get(&()).unwrap().to_string();
    println!("{tree}");
}