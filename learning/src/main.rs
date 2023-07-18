// 导入库
use ferris_says::say;
use std::io::{stdout, BufWriter};
// 导入自己包，先声明同级目录下的rs文件
// 私有模块声明
mod game;

// 共有模块声明
pub mod keywords;
use keywords::{constants, test_mut, Shadowing};

// 这是相对路径
use game::{g, test};
// 只引用一个函数
// use game::g;
// 绝对路径引入，这是根路径
// use crate::game::test;

fn test_says() {
    let stdout: std::io::Stdout = stdout();
    let message: &str = "Hello fellow Rustaceans!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}

// !主程序入口函数
fn main() {
    // 打印
    // println!("Hello, Rust!");
    // 测试函数
    // test_says();
    // 测试导包
    // g();
    // 测试变量和可变性
    test_mut();
}
