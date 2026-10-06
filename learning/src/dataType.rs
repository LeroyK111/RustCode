/*
!标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
长度	有符号	无符号
8-bit	i8  	u8
16-bit	i16 	u16
32-bit	i32 	u32
64-bit	i64 	u64
128-bit	i128	u128
arch	isize	usize

数字字面值	例子
Decimal (十进制)	98_222
Hex (十六进制)	0xff
Octal (八进制)	0o77
Binary (二进制)	0b1111_0000
Byte (单字节字符)(仅限于u8)	b'A'

* 整型溢出，是无法通过编译的。
* 使用 --release flag 在 release 模式中构建时，Rust 不会检测会导致 panic 的整型溢出。相反发生整型溢出时，Rust 会进行一种被称为二进制补码 wrapping（two’s complement wrapping）的操作。
*/
// 使用整型
pub fn change_type() {
    let guess: u32 = "42 ".trim().parse().expect("Not a number!");
    println!("{}", guess)
}

// 浮点数，默认都是双精度，单精度也能用
pub fn float() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}, {y}, {}", x + y)
}

// 数值计算
pub fn count() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum},{difference},{product},{quotient},{truncated},{remainder}");
}

// 布尔值
pub fn bools() {
    let a = true;
    let b = false;

    // 逻辑与（AND）
    let and_result = a && b;
    println!("true && false = {}", and_result); // false

    // 逻辑或（OR）
    let or_result = a || b;
    println!("true || false = {}", or_result); // true

    // 逻辑非（NOT）
    let not_result = !a;
    println!("!true = {}", not_result); // false

    // 条件判断
    if a && !b {
        println!("a 为真 且 b 为假");
    }

    // 作为表达式使用
    let condition = a || b;
    let message = if condition {
        "至少一个为真"
    } else {
        "都为假"
    };
    println!("{}", message);
}

// 字符类型，这个很特殊
pub fn string() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// 复合类型
pub fn tuple() {
    let tup = (500, 6.4, 1);
    // 点索引赋值
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}");

    // 解构赋值
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}

pub fn array() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // 直接给每个元素赋予type
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 分号表示五个元素，五个元素为3: let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    // 通过索引访问数组元素
    let first = a[0];
    let second = a[1];

    println!("{first}");
}

use std::io;

// 通过输入索引，获取数组中的value。如果超过索引范围，则报错
pub fn testArrayIndex() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn print_length2(text: &String) {
    /*
            对照一下：

    机制	检查发生在哪里
    普通的 &T、&mut T 借用规则	编译期
    RefCell<T> 的借用规则	运行时

    RefCell<T> 通过运行时检查换取更灵活的使用方式，因此不能把它和普通借用的检查机制混为一谈。



        */
    println!("{}", text.len());
}

fn main2() {
    let text = String::from("Rust");

    print_length2(&text);

    // 借出去之后，所有权仍然属于 text
    println!("{text}");
}
