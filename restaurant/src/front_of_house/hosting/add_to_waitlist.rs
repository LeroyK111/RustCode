use std::fmt;
use std::io;
// 如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}


// 另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。示例
use std::collections::HashMap;
fn test() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 正常引用
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
