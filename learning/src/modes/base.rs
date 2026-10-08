/*
| 使用位置         | 例子                              | 哪一部分是模式        |
| ------------ | ------------------------------- | -------------- |
| match 分支     | Some(value) => ...              | Some(value)    |
| let 语句       | let (x, y) = point;             | (x, y)         |
| if let 条件    | if let Some(value) = result     | Some(value)    |
| while let 条件 | while let Ok(value) = rx.recv() | Ok(value)      |
| for 循环       | for (index, value) in ...       | (index, value) |
| 函数、闭包参数      | fn show((x, y): (i32, i32))     | (x, y)         |
*/

use std::any::type_name_of_val;
use std::sync::mpsc;

pub fn test() {
    let original = Some(5);

    let result = match original {
        None => None,
        Some(value) => Some(value + 1),
    };

    match original {
        Some(value) => println!("{value}"),
        _ => println!("其他情况"),
    }

    // println!("{result:?}"); // Some(6)
}

pub fn test2() {
    let x = 5;
    let (x, y, _) = (1, 2, 0);
    let (x, ..) = (1, 2, 0);

    let result = Some(5);

    if let Some(value) = result {
        println!("{value}");
    } else {
        println!("没有值");
    }

    let result = Some(5);

    match result {
        Some(value) => {
            println!("{value}");
        }
        None => {
            println!("没有值");
        }
    }
}

pub fn test3() {
    let age: Result<u8, _> = "34".parse();

    if let Ok(age) = age {
        // 这里的 age 是 u8
        println!("{}: {}", type_name_of_val(&age), age)
    }

    // 建立信道
    let (tx, rx) = mpsc::channel::<String>();
    let _ = tx.send(String::from("222"));
    drop(tx);

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}

pub fn test4() {
    let letters = vec!['a', 'b', 'c'];

    for (index, value) in letters.iter().enumerate() {
        println!("{value} 的索引是 {index}");
    }

    let letters: Vec<i32> = vec![1..10].into_iter().flatten().collect();
    println!("{:?}", letters);

    fn show(point: (i32, i32)) {
        let (x, y) = point;

        println!("坐标：({x}, {y})");
    }

    fn show2((x, y): (i32, i32)) {
        println!("坐标：({x}, {y})");
    }

    let sum = |(x, y): (i32, i32)| x + y;

    println!("{}", sum((3, 5))); // 8
}

pub fn test5() {
    let parsedAge: Result<u8, _> = "34".parse();

    if let Ok(ageValue) = parsedAge
        && ageValue > 30
    {
        println!("年龄超过 30");
    }
}
