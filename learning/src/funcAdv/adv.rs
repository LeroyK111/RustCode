/*
写法	含义
fn(i32) -> i32	一种具体的函数指针类型
F: Fn(i32) -> i32	要求类型 F 具有对应的调用能力
-> impl Fn(i32) -> i32	返回一种确定但不公开名字的可调用类型
Box<dyn Fn(i32) -> i32>	用统一的 trait 对象类型保存不同的可调用对象
*/

fn addOne(value: i32) -> i32 {
    value + 1
}

fn doTwice(operation: fn(i32) -> i32, value: i32) -> i32 {
    /*
    operation(5) → addOne(5) → 6
    operation(5) → addOne(5) → 6

    6 + 6 = 12
        */
    operation(value) + operation(value)
}

pub fn test() {
    let answer = doTwice(addOne, 5);

    println!("{answer}");
}

fn addOne2(value: i32) -> i32 {
    value + 1
}

fn doTwice2<F>(operation: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(value) + operation(value)
}

pub fn test2() {
    // 普通函数
    println!("{}", doTwice2(addOne2, 5));

    // 不捕获外部变量的闭包
    println!("{}", doTwice2(|value| value + 1, 5));

    // 捕获外部变量的闭包
    let extra = 10;
    println!("{}", doTwice2(|value| value + extra, 5));
}

pub fn test3() {
    let operation: fn(i32) -> i32 = |value| value + 1;

    println!("{}", operation(5)); // 6
}

// 普通函数
fn call_twice<F>(operation: F)
where
    F: Fn(),
{
    operation();
    operation();
}

pub fn mainx() {
    let message = String::from("hello");

    let print_message = || {
        // 只读取 message，不修改，也不移走
        println!("{message}");
    };

    call_twice(print_message);

    // message 仍然可用
    println!("外部也能读取：{message}");
}

fn call_twice_mut<F>(mut operation: F)
where
    F: FnMut(),
{
    operation();
    operation();
}

pub fn main2x() {
    let mut count = 0;

    let increment = || {
        // 修改捕获的 count
        count += 1;
        println!("count = {count}");
    };

    call_twice_mut(increment);

    println!("最终 count = {count}");
}

fn call_once<F>(operation: F) -> String
where
    F: FnOnce() -> String,
{
    operation()
}

pub fn main3x() {
    let message = String::from("hello");

    let take_message = move || {
        // 把 String 从闭包内部移出去，而不是克隆
        message
    };

    let result = call_once(take_message);

    println!("取出的字符串：{result}");

    // 不能再次使用同一个闭包：
    // let another = call_once(take_message);
}

// map 映射
pub fn test4() {
    let numbers = vec![1, 2, 3];
    // 数据类型转换
    let texts: Vec<String> = numbers.iter().map(|number| number.to_string()).collect();

    let numbers = vec![1, 2, 3];
    let texts: Vec<String> = numbers.iter().map(ToString::to_string).collect();
}

pub fn test5() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let mut statuses: Vec<Status> = (0u32..3).map(Status::Value).collect();

    statuses.push(Status::Stop);

    for status in statuses {
        match status {
            Status::Value(number) => println!("Value({number})"),
            Status::Stop => println!("Stop"),
        }
    }
}

pub fn test6() {
    // 函数 返回 闭包函数
    fn makeAdder(extra: i32) -> impl Fn(i32) -> i32 {
        move |value| value + extra
    }

    let addTen = makeAdder(10);

    println!("{}", addTen(5));
    println!("{}", addTen(8));
}

pub fn test7() {
    // fn makeOne() -> impl Fn(i32) -> i32 {
    //     |value| value + 1
    // }

    // fn makeAdder(extra: i32) -> impl Fn(i32) -> i32 {
    //     move |value| value + extra
    // }

    // !这里不能编译, 就是因为 即使都是闭包类型, 但是指向的位置不是同一个
    // let handlers = vec![makeOne(), makeAdder(123)];

    pub fn test71() {
        fn make_one() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|value| value + 1)
        }

        fn make_adder(extra: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |value| value + extra)
        }

        // 动态分发
        let handlers = vec![make_one(), make_adder(123)];

        for handler in handlers {
            println!("{}", handler(5));
        }
    }
}

/*
| 需求                     | 常见写法                                  |
| ---------------------- | ------------------------------------- |
| 接收普通函数，或可转换为函数指针的非捕获闭包 | operation: fn(i32) -> i32             |
| 接收普通函数和捕获环境的闭包         | F: Fn(i32) -> i32，或按需求使用 FnMut、FnOnce |
| 返回一种确定的闭包类型，但不写出类型名字   | -> impl Fn(i32) -> i32                |
| 用一种集合元素类型保存不同具体闭包      | Box<dyn Fn(i32) -> i32>               |
*/
