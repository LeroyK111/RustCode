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
}

// 布尔值
pub fn bools() {
    let t = true;

    let f: bool = false; // with explicit type annotation
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

    // 解构赋值
    let (x, y, z) = tup;

    // 点索引赋值
    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

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

    // 分号表示五个元素，五个元素为3
    let a = [3; 5];


    // 通过索引访问数组元素
    let first = a[0];
    let second = a[1];


}

use std::io;


// 通过输入索引，获取数组中的value。如果超过索引范围，则报错
fn test() {
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



