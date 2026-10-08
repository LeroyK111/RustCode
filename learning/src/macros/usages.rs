/*
| 类别             | 实现方式                   | 使用形式                  |
| -------------- | ---------------------- | --------------------- |
| 声明宏            | macro_rules!，匹配输入的语法结构 | list![1, 2, 3]        |
| 自定义 derive 过程宏 | 程序读取类型定义，生成附加代码        | #[derive(HelloMacro)] |
| 属性式过程宏         | 程序读取属性参数及被标注的项         | #[route(GET, "/")]    |
| 函数式过程宏         | 程序读取括号中的 token         | sql!(...)             |


编译时：宏生成代码。
运行时：生成的代码执行计算、打印、创建集合等操作。
*/

pub fn test() {
    fn add(first: i32, second: i32) -> i32 {
        first + second
    }
    // 宏的代价是：多了一层“生成代码”的过程，通常比函数更难读、维护和排查问题。
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    macro_rules! greet {
        () => {
            println!("hello");
        };

        ($name:expr) => {
            /*
            $name → 宏中的变量，保存匹配到的代码片段
            expr  → 要求这个代码片段是表达式
            */
            println!("hello {}", $name);
        };
    }

    greet!();
    greet!("Rust");
}

macro_rules! list {
    /*
| 部分            | 含义          |
| ------------- | ----------- |
| $element:expr | 匹配一个表达式     |
| $( ... )      | 把需要重复的模式包起来 |
| ,             | 各次匹配之间用逗号分隔 |
| *             | 重复零次或多次     |
    */
    // $( $element:expr ),* 详解: 左边匹配到几个表达式，这里就生成几条 push。


    // 这就是最初的表达式
    ( $( $element:expr ),* ) => {
        {
            let mut result = Vec::new();

            $(
                result.push($element);
            )*

            result
        }
    };
}

pub fn test2() {
    let numbers = list![1, 2, 3];

    println!("{numbers:?}");
}

// my_library::greet!(); 其他模块可以这样引用
#[macro_export]
macro_rules! greet {
    () => {
        println!("hello");
    };
}

macro_rules! greet2 {
    // 无输入
    () => {
        println!("hello");
    };

    // 一个表达式输入，把它命名为 $name
    ($name:expr) => {
        println!("hello, {}", $name);
    };
}

/*自制宏.md

| 工具         | 职责                       |
| ---------- | ------------------------ |
| proc_macro | 编译器提供的过程宏接口和 TokenStream |
| syn        | 把输入代码解析成便于操作的语法树         |
| quote      | 用模板生成输出代码的 token         |


Rust 宏
├─ 声明宏
│   └─ macro_rules! : 匹配/模板宏
│
└─ 过程宏
    ├─ derive 宏: 追加trait等功能
    ├─ 属性宏: 类似装饰器
    └─ 函数式宏: 高级宏
*/
