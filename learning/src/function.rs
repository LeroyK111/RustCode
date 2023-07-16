// 函数

/*
Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
*/

// 有点python的类型注解的感觉了
pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 表达式写法
pub fn talk() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}


// 返回值函数，只需要箭头函数
fn plus_one(x: i32) -> i32 {
    // 不能加入分号，“mismatched types”（类型不匹配）
    x + 1
}

