/*
使用信号 在 线程间 交换数据
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test() {
    // !创建信道：里面传送的消息类型是 String 本质时转移了所有权
    let (tx, rx) = mpsc::channel::<String>();

    // 创建新线程，把发送端 tx 移进去
    let handle = thread::spawn(move || {
        // 消息在新线程中创建
        let message = String::from("你好，主线程");

        // 把消息发送进信道
        tx.send(message).unwrap();
    });

    println!("主线程：准备接收消息");

    // 没有消息时，主线程停在这里等待
    let received = rx.recv().unwrap();

    println!("主线程收到：{received}");

    // 确保新线程已经结束
    handle.join().unwrap();
}

pub fn test1() {
    //
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        let messages = vec![
            String::from("第一条"),
            String::from("第二条"),
            String::from("第三条"),
        ];

        for message in messages {
            // 每次发送一个 String
            tx.send(message).unwrap();

            thread::sleep(Duration::from_secs(1));
        }

        // 线程闭包结束，tx 被销毁
    });

    // 每次接收一条消息
    // for received in rx {
    //     println!("主线程收到：{received}");
    // }

    // 这样只能时 一次接受一条
    // let received = rx.recv().unwrap();
    // println!("{received:?}");

    while let Ok(received) = rx.recv() {
        println!("主线程收到：{received}");
    }

    println!("所有消息接收完毕");

    handle.join().unwrap();
}

pub fn test2() {
    // 请求方向：主线程 → 新线程
    let (request_tx, request_rx) = mpsc::channel::<String>();

    // 回复方向：新线程 → 主线程
    let (reply_tx, reply_rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        // 新线程拥有 request_rx 和 reply_tx

        // 等待主线程发来请求
        let request = request_rx.recv().unwrap();
        println!("新线程收到：{request}");

        // 把回复发送给主线程
        let reply = String::from("你好，主线程，我收到消息了");
        reply_tx.send(reply).unwrap();
    });

    // 主线程拥有 request_tx 和 reply_rx

    // 先发送请求
    request_tx.send(String::from("你好，新线程")).unwrap();

    // 再等待回复
    let reply = reply_rx.recv().unwrap();
    println!("主线程收到回复：{reply}");

    handle.join().unwrap();
}

pub fn test3() {
    let (tx, rx) = mpsc::channel::<String>();

    // 克隆发送端：tx 和 tx1 对应同一个信道
    let tx1 = tx.clone();

    // 线程 A 拥有 tx1
    let handle_a = thread::spawn(move || {
        let messages = vec![
            String::from("A：第一条"),
            String::from("A：第二条"),
            String::from("A：第三条"),
        ];

        for message in messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // 闭包结束，tx1 被销毁
    });

    // 线程 B 拥有原来的 tx
    let handle_b = thread::spawn(move || {
        let messages = vec![
            String::from("B：第一条"),
            String::from("B：第二条"),
            String::from("B：第三条"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // 闭包结束，tx 被销毁
    });

    // 主线程从同一个接收端获取两边的消息
    for received in rx {
        println!("主线程收到：{received}");
    }

    println!("信道中的消息已经接收完毕");

    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
