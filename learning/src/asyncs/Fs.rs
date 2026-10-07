use std::time::Duration;

pub fn test() {
    trpl::block_on(async {
        // 创建一个独立的异步任务
        let handle = trpl::spawn_task(async {
            println!("任务 A 开始");
            for i in 1..10 {
                println!("任务 A：{}", i);

                // 异步专用定时器
                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("任务 A 完成");
        });

        println!("任务 B 开始");

        // 当前异步任务继续执行自己的循环
        for i in 1..5 {
            println!("任务 B：{}", i);

            trpl::sleep(Duration::from_millis(500)).await;
        }

        println!("任务 B 完成，等待任务 A");

        // 等待任务 A 完成
        handle.await.unwrap();

        println!("两个任务都完成");
    });
}

pub fn test2() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("循环 A：{}", i);

                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("循环 A 完成");
        };

        let fut2 = async {
            for i in 1..5 {
                println!("循环 B：{}", i);

                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("循环 B 完成");
        };

        println!("两个 Future 已创建");

        let results = trpl::join(fut1, fut2).await;

        println!("两个循环都完成");
        println!("返回值：{:?}", results);
    });
}

pub fn test3() {
    trpl::block_on(async {
        // 异步信道
        let (tx, mut rx) = trpl::channel();

        // 单条
        // let message = String::from("你好");
        // tx.send(message).unwrap();
        // message 已经移动进信道，不能再使用
        // let received = rx.recv().await.unwrap();
        // println!("收到：{}", received);

        // 多条发送
        for message in ["第一条", "第二条", "第三条", "第四条"] {
            tx.send(String::from(message)).unwrap();

            trpl::sleep(Duration::from_millis(500)).await;
        }

        // 必须明确销毁发送端
        drop(tx);

        println!("开始接收");

        while let Some(value) = rx.recv().await {
            println!("收到：{}", value);
        }

        println!("接收结束");
    });
}

pub fn test4() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            for message in ["第一条", "第二条", "第三条", "第四条"] {
                tx.send(String::from(message)).unwrap();

                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("发送完成");
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("收到：{}", value);
            }

            println!("接收结束");
        };

        trpl::join(tx_fut, rx_fut).await;

        println!("程序正常结束");
    });
}

pub fn test5() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let sender1 = async move {
            for message in ["A1", "A2", "A3"] {
                tx1.send(String::from(message)).unwrap();

                trpl::sleep(Duration::from_millis(500)).await;
            }

            println!("发送端 A 完成");
        };

        let sender2 = async move {
            for message in ["B1", "B2", "B3"] {
                tx.send(String::from(message)).unwrap();

                trpl::sleep(Duration::from_millis(800)).await;
            }

            println!("发送端 B 完成");
        };

        let receiver = async {
            while let Some(message) = rx.recv().await {
                println!("收到：{}", message);
            }

            println!("所有发送端都已关闭，接收结束");
        };

        trpl::join!(sender1, sender2, receiver);

        println!("全部完成");
    });
}
