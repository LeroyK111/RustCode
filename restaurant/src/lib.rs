// lib.rs

// 全局模块调用
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("called `front_of_house::hosting::add_to_waitlist()`");
        }

        // pub fn seat_at_table() {}
    }

    // pub mod serving {
    //     pub fn take_order() {}

    //     pub fn serve_order() {}

    //     pub fn take_payment() {}
    // }
}

// 路径模块调用
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 引用三方库
// 当前示例暂时没有使用，先保留为学习笔记
// use rand::Rng;

// 引用标准库
// 当前示例暂时没有使用，先保留为学习笔记
// use std::collections::HashMap;

pub fn add(a: i32, b: i32) -> i32 {
    println!("调用lib.rs中的add函数");
    a + b
}

pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

// 补上缺失的私有业务函数
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Breakfast {
    /*
        println!("{}", meal.toast);          // 可以读取
        meal.toast = String::from("Wheat");   // 可以修改，meal 需要可变

        println!("{}", meal.seasonal_fruit);  // 不可以读取
        meal.seasonal_fruit = ...;            // 不可以修改

        补充说明：
        上面的“不可以”针对定义 Breakfast 的模块外部。
        在定义它的模块内部，仍然可以访问私有字段。
    */
    pub toast: String,      // 对外开放
    seasonal_fruit: String, // 内部管理
}

/*
原来的随机数练习暂时保留在注释中：

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
*/

// lib 两种调用方式, main引入调用 or cargo test 测试调用
// cargo test --lib -- --nocapture               测试全部函数,并且打印测试结果
// cargo test --lib it_works                    仅仅测试 it_works 函数

/*
补充说明：

1. “测试全部函数”更准确地说是：
   运行库目标中被标记为 #[test]、且未被忽略或过滤的测试函数，
   不是自动测试每一个普通函数。

2. it_works 默认是名称过滤词，不是严格的完整名称匹配。

3. --nocapture 是旧别名，当前推荐：
   cargo test --lib -- --no-capture
*/

/*
todo

cargo test --help 查看 Cargo 的选项
cargo test -- --help 查看测试执行程序的选项

*/

#[cfg(test)]
mod tests {
    /*
        | 语法               | 含义                           |
        | ------------------ | ------------------------------ |
        | #[cfg(test)]       | 只在测试配置下编译这个模块       |
        | mod tests          | 定义一个名为 tests 的模块       |
        | use super::*;      | 把父模块中的可用项引入当前作用域 |
        | #[test]            | 标记下面的函数是测试用例         |
        | assert_eq!(...)    | 检查两个值是否相等，不相等就让测试失败 |
    */

    // 保留原来的显式导入，并补充私有函数
    use super::{add, internal_adder};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        println!("测试结果：{}", result);
        assert_eq!(result, 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn additionWorks() {
        // 准备输入
        let a = 2;
        let b = 2;

        // 调用实际函数
        let result = add(a, b);

        // 检查结果
        assert_eq!(result, 4);
    }

    // 补充：单元测试可以直接验证父模块中的私有函数
    #[test]
    fn internal_adder_works() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    /*
        | 你要验证什么       | 写法                         | 通过条件     |
        | ------------------ | ---------------------------- | ------------ |
        | 某个条件成立       | assert!(condition)           | 条件为 true  |
        | 某个条件不成立     | assert!(!condition)          | 原条件为 false |
        | 得到了准确结果     | assert_eq!(actual, expected) | 两者相等     |
        | 结果不是某个值     | assert_ne!(actual, unwanted) | 两者不相等   |
    */

    #[test]
    fn ass() {
        assert!(10 > 5);
        assert!(!"hello".is_empty());
        assert_eq!(add(2, 3), 5);
        assert_ne!(add(2, 3), 0, "这里则是类似于 expect 了");

        /*
        补充说明：
        类似之处是都可以提供失败说明。
        但 assert_ne! 检查两个值不相等；
        expect 用于取出 Option / Result 中的值。
        */

        // 举个例子
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        assert_eq!(
            Point { x: 3, y: 4 },
            Point { x: 3, y: 4 }
        );
    }

    // 这里保留为测试模块内的辅助函数
    fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            Err("除数不能为零")
        } else {
            Ok(a / b)
        }
    }

    // 预期成功 则需要使用?
    // 补充说明：可以使用 ?，不是必须使用。
    #[test]
    fn divide_success_with_question_mark() -> Result<(), &'static str> {
        // 无论成功or失败 都交给result处理
        /*
        补充说明：
        成功：? 取出值，赋给 result。
        失败：? 提前返回 Err，测试失败，
              不会执行后面的断言。
        */
        let result = divide(10, 2)?;

        // 5就成功, 其他则失败
        assert_eq!(result, 5);

        Ok(())
    }

    // 预期失败 则不用?
    #[test]
    fn divide_by_zero_is_expectsed() {
        // 直接失败, 结果给result
        // 补充说明：这里是返回 Err，不是发生 panic。
        let result = divide(10, 0);

        // 失败了则通过测试
        // 更准确地说：返回 Err 符合预期，因此断言通过。
        assert!(result.is_err());
    }

    fn require_positive(value: i32) {
        assert!(value > 0, "必须为正数");
    }

    #[test]
    #[should_panic(expected = "必须为正数")]
    fn rejects_zero() {
        require_positive(0);
    }
}