/*
可反驳实例
*/

pub fn test() -> i32 {
    let result: Option<i32> = Some(5);
    // let a = result;

    // match result {
    //     Some(x) => {
    //         println!("拿到数字：{x}");
    //     }
    //     None => {
    //         println!("没有数字");
    //     }
    // }

    // 实例
    let Some(x) = result else { return 0 };

    // todo 如果return有返回值,则整个函数都是 Option<>
    x
}

pub fn test2() {
    /*
    筛选、绑定、解构、忽略: 这就是四种模式用法
    | 写法             | 含义               |
    | -------------- | ---------------- |
    | 5              | 必须等于 5           |
    | value          | 创建变量，接住当前值       |
    | 1 \\\| 2       | 等于 1 或 2         |
    | 1..=5          | 在 1 到 5 之间，包含两端  |
    | Some(value)    | 要求是 Some，并取出里面的值 |
    | Point { x, y } | 拆开结构体字段          |
    | _              | 匹配，但不绑定变量        |
    | ..             | 忽略剩余部分           |
    | 模式 if 条件       | 模式匹配后，再检查条件      |
    | value @ 模式     | 既检查模式，又把该部分绑定到变量 |

        */

    let number = 5;

    // match number {
    //     0 => println!("等于 0"),
    //     1 | 2 => println!("等于 1 或 2"),
    //     3..=5 => println!("在 3 到 5 之间"),
    //     _ => println!("其他数字"),
    // }

    // match 'c' {
    //     'a'..='z' => println!("小写 ASCII 字母"),
    //     _ => println!("其他字符"),
    // }

    // todo 这里就是最具有误导的地方了
    let result = Some(5);
    let y = 10;

    // 对于类属性而言, 只判断type 不判断 value
    match result {
        // Some(50) => println!("里面是 50"),
        // Some(y) => println!("里面是 {y}"),
        Some(value) if value == y => println!("里面是 {value}"),
        None => println!("没有值"),
        Some(value) => println!("打印value: {value}"),
    }

    println!("外面的 y 仍然是 {y}");
}

pub fn test3() {
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = point;

    println!("a = {a}，b = {b}");

    let Point { x, y } = point;
}

pub fn test4() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let message = Message::ChangeColor(0, 160, 255);

    match message {
        Message::Quit => {
            println!("退出");
        }
        Message::Move { x, y } => {
            println!("移动到 ({x}, {y})");
        }
        Message::Write(text) => {
            println!("文字：{text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("颜色：({r}, {g}, {b})");
        }
    }
}

pub fn test5() {
    let numbers = (2, 4, 8, 16, 32);

    let (first, _, third, _, fifth) = numbers;

    println!("{first}，{third}，{fifth}");

    let numbers = (2, 4, 8, 16, 32);

    let (first, .., last) = numbers;

    println!("{first}，{last}");

    let text = Some(String::from("Hello"));

    match text {
        // _下划线变量 告诉编译器 text 被移动了
        Some(_content) => println!("有文字"),
        None => {}
    }

    // 报错：内部的 String 已经移动到 _content
    // println!("{text:?}");
}

pub fn test6() {
    let number = Some(4);

    match number {
        Some(value) if value % 2 == 0 => {
            println!("{value} 是偶数");
        }
        Some(value) => {
            println!("{value} 是奇数");
        }
        None => {}
    }

    let number = 4;
    let enabled = false;

    match number {
        // (number 是 4、5、6 中的一个) 并且 enabled 为 true
        4 | 5 | 6 if enabled => println!("允许"),
        _ => println!("不允许"),
    }
}

pub fn test7() {
    /*
    | 写法                  | 是否筛选范围 | 是否创建变量 |
    | ------------------- | ------ | ------ |
    | id: 3..=7           | 是      | 否      |
    | id: idValue         | 否      | 是      |
    | id: idValue @ 3..=7 | 是      | 是      |
    */
    let number = 5;

    // @ 就是创建变量并筛选
    match number {
        value @ 3..=7 => {
            println!("在范围内，具体值是 {value}");
        }
        _ => {
            println!("不在范围内");
        }
    }
}
