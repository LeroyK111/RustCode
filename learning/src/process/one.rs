use std::thread;
use std::time::Duration;

pub fn test() {
    println!("主线程：准备创建新线程");

    let handle = thread::spawn(|| {
        /*
          闭包交给 新线程执行的代码
        */

        for i in 1..10 {
            println!("新线程：{i}");

            // 等待1ms
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("主线程：已经创建新线程，继续往下执行");
    thread::sleep(Duration::from_millis(1));

    for i in 1..5 {
        println!("主线程：{i}");

        thread::sleep(Duration::from_millis(1));
    }

    // spawn() 启动线程，join() 等待对应线程；把等待放在哪里，会影响后续代码什么时候执行。
    // 主线程必须等待子线程 完成
    handle.join().unwrap();

    println!("主线程：准备结束");
}

pub fn test1() {
    let v = vec![1, 2, 3];

    // 线程间 变量和实例 需要特殊方式才能共享
    // move 就是将v的所有权转移到了 子线程内部
    let handle = thread::spawn(move || {
        println!("新线程收到：{v:?}");
    });

    // println!("{v:?}");
    // drop(v);

    handle.join().unwrap();
    // println!("{v:?}");
}
