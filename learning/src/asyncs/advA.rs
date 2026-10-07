use std::thread;
use std::time::Duration;

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));

    println!("{} 执行了 {}ms", name, ms);
}

pub fn test() {
    trpl::block_on(async {
        let a = async {
            println!("A 开始");

            // 同步代码 就是会阻塞
            slow("A", 30);
            slow("A", 10);
            slow("A", 20);

            // !注释这行 会导致 B根本无法开始
            // trpl::sleep(Duration::from_millis(50)).await;

            println!("A 完成");
        };

        let b = async {
            // 只有当 A中 有方法await后, B才能介入执行
            println!("B 开始");

            slow("B", 75);
            slow("B", 10);
            slow("B", 15);
            slow("B", 350);

            trpl::sleep(Duration::from_millis(50)).await;

            println!("B 完成");
        };

        let _winner = trpl::select(a, b).await;

        println!("竞争结束");
    });
}

fn slow1(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{} 执行了 {}ms", name, ms);
}

pub fn test2() {
    /*
        这一段工作做完了，允许其他工作获得执行机会。
        async sleep 可以切换
        yield_now() 也可以切换

    */
    trpl::block_on(async {
        // let one_ms = Duration::from_millis(1);

        let a = async {
            println!("A 开始");

            slow1("A", 30);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow1("A", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow1("A", 20);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            println!("A 完成");
        };

        let b = async {
            println!("B 开始");

            slow1("B", 75);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow1("B", 10);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow1("B", 15);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            slow1("B", 350);
            // trpl::sleep(one_ms).await;
            trpl::yield_now().await;

            println!("B 完成");
        };

        let _winner = trpl::select(a, b).await;

        println!("竞争结束");
    });
}

use std::future::Future;
use trpl::Either;

// 超时任务timeout 判断
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    // 让工作与计时器竞争
    let winner = trpl::select(future_to_try, trpl::sleep(max_time)).await;

    match winner {
        // 第一个参数是工作 Future
        Either::Left(output) => Ok(output),

        // 第二个参数是计时器，输出为 ()
        Either::Right(_) => Err(max_time),
    }
}

pub fn test3() {
    trpl::block_on(async {
        let work = async {
            println!("工作开始");

            trpl::sleep(Duration::from_secs(5)).await;

            println!("工作完成");

            "任务的最终结果"
        };

        let result = timeout(work, Duration::from_secs(2)).await;

        match result {
            Ok(message) => {
                println!("成功：{}", message);
            }
            Err(limit) => {
                println!("超时，时间限制为 {} 秒", limit.as_secs());
            }
        }
    });
}
