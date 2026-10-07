use std::thread;

pub fn test() {
    let message = String::from("交给子线程");

    let handle = thread::spawn(move || {
        println!("{}", message);

        // 返回值不要分号
        message
    });

    let result = handle.join().unwrap();

    // 所有权已经移动，不能再使用 message
    // println!("{}", message);
    println!("{}", result);
}
