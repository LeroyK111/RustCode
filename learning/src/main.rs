// 导入库
use ferris_says::say;
use std::io::{self, stdout, BufWriter};

// 导入自己包，先声明同级目录下的rs文件
// 私有模块声明
mod game;

// 共有模块声明
// mod variablesVariability;
// use variablesVariability::{constants, test_mut, Shadowing};

// 这是相对路径
// use game::{g, test};
// 只引用一个函数
// use game::g;

// 绝对路径引入，这是根路径
// use crate::game::test;

// 测试中文包，不支持😄
// mod 数据类型;
// use 数据类型::{changeType;}

// mod function;
// use function::print_labeled_measurement;

// mod dataType;
// use dataType::{array, bools, changeType, count, float, string, tuple};

// 测试所有权
// mod ownership;
// use ownership::{scope, scope1, scope2, scope3, scope4};
// 引用和借用
// mod referenceBorrowing;
// use referenceBorrowing::demo;

// 结构体
// mod structure;
// use structure::theWorld;

// 枚举体
// mod enums;
// use enums::{enums, enums1, enums2, enums3};

// 集合
// mod coll;
// use coll::hashmaps::hashmaps;
// use coll::strings::strMain;
// use coll::vectors::{call, call2};

fn test_says() {
    let stdout: std::io::Stdout = stdout();
    let message: &str = "Hello fellow Rustaceans!";
    let width: usize = message.chars().count();

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());
    say(message, width, &mut writer).unwrap();
}

fn test_guess() {
    // 提示
    println!("Please input your guess.");
    // 创建可变值， 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。
    let mut guess = String::new();

    // 这是不可变值
    // let apples = 5;aaq
    // mut 在rust中所有对象默认都是不可变的
    // let mut apples1 = 5;

    // 引用io标准输入 句柄
    io::stdin()
        // 读取字符串 并引用 &mut guess 赋值
        .read_line(&mut guess)
        // 异常捕获, 如果不调异常捕获，程序可以执行，但是会报错提示
        .expect("Failed to read line");

    // 将字符串转换为 i32（32 位整数）
    let z: i32 = match guess.trim().parse() {
        Ok(n) => n,
        Err(e) => {
            println!("错误代码{}", e);
            // 主main中不支持 return
            -10
        }
    }; 

    // 打印字符串，模版语法
    println!("You guessed: {guess}");

    // 设置两个常量
    let x = 5;
    let y = 10;

    // 模版语法
    println!("x = {x} and y + z = {}", y + z);
}

// !主程序入口函数
fn main() {
    // 打印
    // println!("Hello, Rust!");
    // 测试猜测
    test_guess();
    // 测试函数
    // test_says();

    // 测试导包
    // g();

    // *测试变量
    // test_mut();
    // 测试不可变量
    // constants()
    // 测试定义域
    // Shadowing();

    // *改变数据类型
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

    // todo 作用域函数，打印
    // scope()
    // 字符串类型
    // scope1();
    // scope2();
    // 字面量，深复制、克隆
    // scope3();
    // 函数和字面量
    // scope4();

    // *所有权：引用和借用
    // demo();

    // ?结构体，有点像typescript的接口语法
    // theWorld();

    // todo 枚举
    // enums();
    // enums1();
    // enums2();
    // enums3();

    // !集合
    // call();
    // call2();
    // strMain();
    // hashmaps();
}
