// 导入库
use ferris_says::say;
use std::io::{stdout, BufWriter};
// 导入自己包，先声明同级目录下的rs文件
// 私有模块声明
mod game;

// 共有模块声明
mod variablesVariability;
use variablesVariability::{constants, test_mut, Shadowing};

// 这是相对路径
use game::{g, test};
// 只引用一个函数
// use game::g;
// 绝对路径引入，这是根路径
// use crate::game::test;

// 测试中文包，不支持😄
// mod 数据类型;
// use 数据类型::{changeType;}


mod function;
use function::{print_labeled_measurement};

mod dataType;
use dataType::{array, bools, changeType, count, float, string, tuple};

fn test_says() {
    let stdout: std::io::Stdout = stdout();
    let message: &str = "Hello fellow Rustaceans!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}


// 测试所有权
mod ownership;
use ownership::{scope, scope1};


// !主程序入口函数
fn main() {
    // 打印
    // println!("Hello, Rust!");
    // 测试函数
    // test_says();
    // 测试导包
    // g();
    // 测试变量
    // test_mut();
    // 测试不可变量
    // constants()
    // 测试定义域
    // Shadowing();

    // 改变数据类型
    // changeType()
    // 标量类型
    // 浮点
    // float();
    // 计算
    // count();
    // 布尔值
    // bools();
    // 字符串
    // string();
    // 元组
    // tuple();
    // 数组
    // array();

    // 函数
    // print_labeled_measurement(32, '2');

    // 作用域函数，打印
    // scope()
    scope1();

}
