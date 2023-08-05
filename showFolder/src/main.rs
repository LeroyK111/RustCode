// crate 是当前项目的根文件, 对于一个二进制 crate 而言是src/main.rs
// gardan的代码路径
// use crate::garden::vegetables::Asparagus
// 申明子模块的路径
mod garden;

// 调用fs文件系统
// use std::fs::File
// use std::error::Error
// 调用异常捕获
mod err;
// use err::demo;

// 查看数据类型
use std::any::type_name;


// 引入traits
mod traits;
use traits::interfaces;

// 生命周期
mod lefttime;
use lefttime::timer;

// ?这里是主函数报错捕获
// use std::io::ErrorKind;
// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    // !模块路径知识学习完毕
    // let plant = Asparagus {};
    // println!("I'm growing {:?}!", plant);
    // my::indirect_call();

    // !这里我们学习panic 不可恢复错误
    // panic!("crash and burn");
    /*
    这里尝试访问 vector 的第一百个元素（这里的索引是 99 因为索引从 0 开始），不过它只有三个元素。这种情况下 Rust 会 panic。[] 应当返回一个元素，不过如果传递了一个无效索引，就没有可供 Rust 返回的正确的元素。

    *C 语言中，尝试读取数据结构之后的值是未定义行为（undefined behavior）。你会得到任何对应数据结构中这个元素的内存位置的值，甚至是这些内存并不属于这个数据结构的情况。这被称为 缓冲区溢出（buffer overread），并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取储存在数据结构之后不被允许的数据。

    为了保护程序远离这类漏洞，如果尝试读取一个索引不存在的元素，Rust 会停止执行并拒绝继续。尝试运行上面的程序会出现如下：
    */
    // let v = vec![1, 2, 3, 4];
    // v[99];
    // ? shell执行 RUST_BACKTRACE=1 cargo run 即可获取更详细的报错内容

    // !处理result，可恢复错误
    // ?本质是利用了枚举成功or失败
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // 打开文件
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     // 这里则是通用异常
    //     // Err(error) => panic!("Problem opening the file: {:?}", error),
    //     // 枚举再细化，区分不同的错误
    //     Err(error) => match error.kind() {
    //         // 找不到文件，则创建文件
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         // 其他异常就爆出
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // * 更好的写法，利用闭包
    // 利用链式调用
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // ? 链式语法，类似promise then cache的异常处理方法
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    // ? 这里引用err.rs中的其他api
    // demo();

    // todo 哪怕是主函数，也可以进行错误传递
    // let greeting_file_result = File::open("hello.txt")?;
    // Ok(());

    // !泛型
    // let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];
    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);
    // * 抽象层
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);
    // * 传入泛型，由输入形参决定类型
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // *结构体中泛型
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // };
    // #[derive(Debug)]
    // struct Point1<T, U> {
    //     x: T,
    //     y: U,
    // }
    // impl<T, U> Point1<T, U> {
    //     fn x(&self) -> &T {
    //         &self.x
    //     }
    // }
    // 强制制定泛型
    // impl Point<f32> {
    //     fn distance_from_origin(&self) -> f32 {
    //         (self.x.powi(2) + self.y.powi(2)).sqrt()
    //     }

    // }
    // let wont_work = Point { x: 0, y: 4.0 }; // 会报错，类型不定
    // let wont_work = Point1 { x: 0, y: 4.0 }; // 编译通过，
    // println!("{:?}", wont_work);
    // * 枚举泛型
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // * 泛型嵌套
    // struct Point<X1, Y1> {
    //     x: X1,
    //     y: Y1,
    // }

    // impl<X1, Y1> Point<X1, Y1> {
    //     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
    //         Point {
    //             x: self.x,
    //             y: other.y,
    //         }
    //     }
    // }
    // let p1 = Point { x: 5, y: 10.4 };
    // let p2 = Point { x: "Hello", y: 'c' };
    // let p3 = p1.mixup(p2);
    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let integer = Some(5);
    // let float = Some(5.0);
    // *Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
    // let integer = Some(5);
    // let float = Some(5.0);
    // println!("{:?} is {}", integer, type_of(integer));
    // println!("{:?} is {}", integer, type_of(float));

    // !trait定义共同行为
    // interfaces();

    // !生命周期
    timer();
}

fn type_of<T>(_: T) -> &'static str {
    // 通过泛型，获取类型 
    type_name::<T>()
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn function() {
    println!("called `function()`");
}

// module cool 模块cool
mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

// 模块my
mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为 `function` 的函数！
        print!("called `my::indirect_call()`, that\n> ");

        // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
        // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
        // 因为他们表示相同的函数。
        self::function();
        function();

        // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
        self::cool::function();

        // `super` 关键字表示父作用域（在 `my` 模块外面）。
        super::function();

        // 这将在 *crate* 作用域内绑定 `cool::function` 。
        // 在这个例子中，crate 作用域是最外面的作用域。
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

// 单个引用
// use std::cmp::Ordering;
// 引用上一级
// use std::io;
// 嵌套引用
// use std::{cmp::Ordering, io};
// 单个引用
// use std::io::Write;
// 括号批量引用
// use std::io::{self, Write};
// 全部引用
// use std::collections::*;
