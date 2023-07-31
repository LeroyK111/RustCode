// 全局模块调用
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

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
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

