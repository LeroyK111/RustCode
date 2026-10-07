use std::sync::{Arc, Mutex};
use std::thread;
pub fn test() {
    // 创建互斥器，内部保存数字 5
    let m = Mutex::new(5);

    {
        // 获取锁，得到访问内部数据的守卫
        let mut num = m.lock().unwrap();

        println!("修改前：{}", *num);

        // 通过守卫修改内部数字
        *num = 6;

        println!("修改后：{}", *num);
    } // num 被销毁，自动释放锁

    println!("互斥器：{m:?}");
}

pub fn test1() {
    // 主线程拥有一个共享句柄
    // 内部只有一份 Mutex，Mutex 内部只有一份数字
    let counter = Arc::new(Mutex::new(0));

    // 保存每个线程的句柄
    let mut handles = Vec::new();

    for _ in 0..10 {
        // 为这个线程创建一个共享句柄
        let thread_counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // 获取同一把锁
            let mut num = thread_counter.lock().unwrap();

            // 修改同一个数字
            *num += 1;

            // 闭包结束，num 被销毁，自动释放锁
            // thread_counter 也随之销毁

            // drop(num); 主动调用也可以释放锁
        });

        handles.push(handle);
    }

    println!("所有者数量: {:?}", Arc::strong_count(&counter));

    // 等待全部十个线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 最后主线程加锁，读取结果
    let result = counter.lock().unwrap();

    println!("Result: {}", *result);
}
