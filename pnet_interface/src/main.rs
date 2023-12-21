use pnet::datalink;

/*
探索如何使用Rust中的pnet库查找和列出计算机上的网络接口。

当你从事底层网络编程任务(如数据包捕获或自定义数据包创建)时，了解网络接口是至关重要的。
*/

fn main() {
    println!("Available interfaces:");
    let interfaces = datalink::interfaces();
    for (index, interface) in interfaces.iter().enumerate() {
        println!("{}: {}", index + 1, interface.name);
    }
}
