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
use rand::Rng;
// 引用标准库
use std::collections::HashMap;

pub fn add(a: i32, b: i32) -> i32 {
    println!("调用lib.rs中的add函数");
    a + b
}

// lib 两种调用方式, main引入调用 or cargo test 测试调用
// cargo test --lib -- --nocapture               测试全部函数,并且打印测试结果
// cargo test --lib it_works                     仅仅测试 it_works 函数
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        println!("测试结果：{}", result);
        assert_eq!(result, 4);
    }

    use super::add;

    #[test]
    fn additionWorks() {
        // 准备输入
        let a = 2;
        let b = 2;

        // 调用实际函数
        let result = add(a, b);

        // 检查结果
        assert_eq!(result, 4);
    }
}

pub struct Breakfast {
    /*
        println!("{}", meal.toast);              // 可以读取
    meal.toast = String::from("Wheat");      // 可以修改，meal 需要可变

    println!("{}", meal.seasonal_fruit);  // 不可以读取
    meal.seasonal_fruit = ...;            // 不可以修改
        */
    pub toast: String,      // 对外开放
    seasonal_fruit: String, // 内部管理
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
