/*
| 内容               | 解决的问题                |
| ---------------- | -------------------- |
| newtype          | 怎样让编译器区分本来容易混淆的数据？   |
| 类型别名             | 怎样把冗长的类型写得简洁？        |
| never 类型 !       | 不会正常完成的表达式，怎样参与类型检查？ |
| DST、Sized、?Sized | 类型大小不固定时，怎样使用和传递它？   |
*/

// !创建了新的结构体
struct Millimeters(u32);
struct Meters(u32);

fn showLength(length: Millimeters) {
    println!("长度是 {} 毫米", length.0);
}

// 或者可以用枚举+match
// 泛型 + impl 共用方法

pub fn test() {
    showLength(Millimeters(500));

    // 下面两行都不能编译：
    // showLength(Meters(2));
    // showLength(500);

    let meters = Meters(2);

    let millimeters = Millimeters(meters.0 * 1000);

    showLength(millimeters);
}

// !类型别名
type Kilometers = i32;

pub fn test2() {
    let first: i32 = 5;
    let second: Kilometers = 10;

    println!("{}", first + second);
}

// 给“装在 Box 中、可跨线程转移、不借用短生命周期数据的闭包”起别名
type Thunk = Box<dyn Fn() + Send + 'static>;

fn run(task: Thunk) {
    println!("准备执行任务");
    task();
}

fn create_task() -> Thunk {
    Box::new(|| println!("执行任务"))
}

pub fn test3() {
    let task = create_task();
    run(task);
}

// todo never类型

fn first() -> () {
    println!("完成");
}

fn second() -> ! {
    panic!("不能继续");
}

pub fn test4() {
    /*
        | 返回类型 | 含义             |
    | ---- | -------------- |
    | ()   | 函数可以正常结束，返回单位值 |
    | !    | 函数不会正常返回给调用位置  |
        */
    let result = first();
    let second = second();
}

fn getNumber(result: Option<i32>) -> i32 {
    let number = match result {
        Some(value) => value,
        // return 0 这个表达式不会正常完成，其类型是 !；它携带的 0 则必须符合函数的返回类型。这是两个不同层面的要求
        None => return 0,
    };

    number + 1
}

fn otherStr() {
    let inputs = ["12", "abc", "34"];

    for input in inputs {
        let number: u32 = match input.parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!("有效数字：{number}");
    }
}

/*
| 类型             | 是否为 Sized | 原因            |
| -------------- | --------- | ------------- |
| str            | 否         | 字符串数据长度不固定    |
| &str           | 是         | 引用及其元信息的大小固定  |
| String         | 是         | 管理结构大小固定      |
| [i32]          | 否         | 切片元素数量不固定     |
| [i32; 3]       | 是         | 元素数量已经写进类型    |
| &[i32]         | 是         | 切片引用大小固定      |
| dyn Trait      | 否         | 具体实现类型被隐藏     |
| Box<dyn Trait> | 是         | 这个指针及其元信息大小固定 |
*/

pub fn test5() {
    /*
    text 句柄本身就包含了
        指针
        长度
        容量
    */
    let text = String::from("hello");
}

use std::fmt::Display;

// Display 管怎么打印，Sized 管类型大小是否已知，?Sized 允许两种情况。
fn show<T: Display + ?Sized>(value: &T) {
    println!("{value}");
}

pub fn test6() {
    show("hello");

    show(&123);
}

/*
newtype → 创建类型边界
type 别名 → 简化已有类型的名字
! → 这条执行路径不正常完成
?Sized → 允许目标类型没有编译时固定大小
*/
