use trpl::StreamExt;

use std::time::Duration;
use trpl::{ReceiverStream, Stream};

pub fn test() {
    trpl::block_on(async {
        let values: Vec<i32> = (1..=10).collect();

        // 创建迭代器，每次取值时将数字乘以 2
        let iter = values.iter().map(|n| n * 2);

        // 将迭代器包装成 Stream
        let mut stream = trpl::stream_from_iter(iter);

        // 逐个取得 Stream 的元素
        while let Some(value) = stream.next().await {
            println!("The value was: {}", value);
        }

        println!("流结束");
    });
}

pub fn test2() {
    trpl::block_on(async {
        let mut stream = trpl::stream_from_iter(1..=10)
            .map(|number| number * 2)
            .filter(|number| number % 3 == 0 || number % 5 == 0)
            .take(3);

        /*
                原始数据：1、2、3、4、5、6、7、8、9、10
            ↓ map：乘以 2
        2、4、6、8、10、12、14、16、18、20
            ↓ filter：保留 3 或 5 的倍数
        6、10、12、18、20
            ↓ take：最多取 3 个
        6、10、12
                */

        while let Some(value) = stream.next().await {
            println!("{}", value);
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        for message in ["第一条", "第二条", "第三条"] {
            trpl::sleep(Duration::from_millis(500)).await;

            if tx.send(message.to_string()).is_err() {
                // 接收端已经关闭，不再发送
                break;
            }
        }
    });

    // 返回的是专门包装信道接收端的流类型，
    ReceiverStream::new(rx)
}

pub fn test3() {
    trpl::block_on(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("收到：{}", message);
        }

        // 毕竟是 item
        // let messages: Vec<String> = get_messages().collect().await;

        // for message in messages {
        //     println!("收到：{}", message);
        // }

        println!("消息流结束");
    });
}
