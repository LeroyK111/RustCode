// 包装异步函数
async fn calculate() -> u32 {
    println!("3. 开始执行 calculate");

    let result = 10 + 20;

    println!("4. calculate 准备返回结果");

    result
}

//  cargo add trpl  准备一个简单的异步库

/*
| 部分                       | 作用                      |
| ------------------------ | ----------------------- |
| async fn / async { ... } | 构造异步计算对应的 Future        |
| .await                   | 在异步代码中等待一个 Future 的输出   |
| trpl::block_on(...)      | 建立运行时，驱动传入的 Future 直到完成 |
*/
pub fn test() {
    println!("1. 准备调用 calculate");

    // 创建一个待执行的async task
    let future = calculate();

    println!("2. 已创建 Future，但函数体还没有执行");

    // block_on 会阻塞调用它的当前线程，直到传入的 Future 完成；但运行时内部可以协调异步任务。它是普通同步入口与异步代码之间的连接点。
    // 异步函数只能在 异步函数/异步块中 才能执行
    trpl::block_on(async {
        let result = future.await;

        println!("5. 得到结果：{}", result);
    });

    println!("6. 异步代码执行完成");
}

use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    // 第一次 await：等待取得 HTTP 响应
    let response = trpl::get(url).await;

    // 第二次 await：等待取得完整的响应文本
    let response_text = response.text().await;

    // 解析 HTML，寻找第一个 title 元素
    let html = Html::parse(&response_text);

    html.select_first("title").map(|title| title.inner_html())
}

pub fn test2() {
    //  cargo run -- https://www.google.com
    let args: Vec<String> = std::env::args().collect();

    // 比原文多加一个参数检查，避免没有参数时越界
    let Some(url) = args.get(1) else {
        println!("用法：cargo run -- <URL>");
        return;
    };

    trpl::block_on(async {
        println!("准备获取网页标题");

        let title = page_title(url).await;

        match title {
            Some(text) => println!("网页标题：{}", text),
            None => println!("这个网页没有 title 元素"),
        }
    });
}

use trpl::Either;

async fn page_title2(url: &str) -> (&str, Option<String>) {
    println!("开始获取：{}", url);

    let response = trpl::get(url).await;
    let response_text = response.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|element| element.inner_html());

    // 返回 URL，以及可能存在的标题
    (url, title)
}

pub fn test3() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("用法：cargo run -- <URL1> <URL2>");
        return;
    }

    trpl::block_on(async {
        // 这里只创建 Future，函数体还没有执行
        let future1 = page_title2(&args[1]);
        let future2 = page_title2(&args[2]);

        println!("两个 Future 已创建，开始竞争");

        // !并发推进两个 Future，取得先完成的输出: 再次包装
        // ! select 是选择最快的函数 返回结果.
        // let winner = trpl::select(future1, future2).await;

        // let (url, title) = match winner {
        //     Either::Left(result) => result,
        //     Either::Right(result) => result,
        // };

        // println!("先完成的是：{}", url);

        // match title {
        //     Some(text) => println!("网页标题：{}", text),
        //     None => println!("这个网页没有 title 元素"),
        // }

        // !使用join则是 等待两个
        let (result1, result2) = trpl::join(future1, future2).await;

        // println!("两个网页都处理完成了");

        let (url1, title1) = result1;
        let (url2, title2) = result2;
        let t1 = title1.unwrap();
        let t2 = title2.unwrap();

        //  cargo run -- https://www.google.com https://www.bing.com
        println!("{url1}:{t1}");
        println!("{url2}:{t2}");
    });
}
