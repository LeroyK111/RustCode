// 函数

/*
Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
*/

// 有点python的类型注解的感觉了
pub fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 表达式写法
pub fn talk(x: i32) {
    // let y = 6 语句不返回值，所以 x 没有可以绑定到的东西。这与 C 和 Ruby 等其他语言不同，在这些语言中，赋值会返回赋值的值。在这些语言中，你可以写 x = y = 6 ，让 x 和 y 都具有值 6 ；在 Rust 中则不是这种情况。
    // let x = (let y = 6);

    // 表达式
    let y = {
        // 其他语言则需要return
        x + 1
    };
    println!("The value of y is: {y}");
}

// 返回值函数，只需要箭头函数
pub fn plus_one(x: i32) -> i32 {
    // 不能加入分号，“mismatched types”（类型不匹配)
    // 但语句的计算结果不为值，该值由 Unit（） 表示。
    x + 1
}

// if分支
pub fn if_main() {
    let number = 3;
    if number > 5 {
        println!("满足条件则为 true");
    } else if number % 3 == 0 {
        println!("3的倍数");
    } else {
        println!("不满足条件则为 false");
    }

    if number != 0 {
        println!("不等于");
    }

    // 简写赋值
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("赋值写法 {number}");
}

// 使用循环loop, while, and for
pub fn testLoop() {
    let mut count = 0;
    let result = loop {
        if count > 5 {
            break "ok!";
        }
        count += 1;
    };
    println!("{count}, {result}")
}

pub fn testLoop1() {
    let mut count = 0;
    // 循环嵌套记得打tag
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // 指定跳出的循环tag
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


pub fn testWhile() {
    let mut number = 3;

    while number != 0 {
        // 不满足条件则，一直循环
        println!("{number}!");

        number -= 1;
    }

    println!("done! {number}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}


pub fn testFor() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}


pub fn testFor1() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}



// 一等函数
pub fn test1() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let mut sum = add;

    sum(1, 2); // Returns 3
}

// 一等函数
// fn test2() {
//     fn call_with_two(func: fn(i32, i32) -> i32, x: i32) -> i32 {
//         func(x, 2)
//     }

//     call_with_two(add, 1); // Returns 3
// }

// 一等函数
// fn test3() {
//     fn make_adder(x: i32) -> fn(i32) -> i32 {
//         fn add(y: i32) -> i32 {
//             x + y
//         }
//         add
//     }

//     let add_10 = make_adder(10);

//     add_10(3); // Returns 13// Returns 3
// }

// fn test4() {
//     // 迭代器
//     let vec = vec![1, 2, 3];
//     let doubled = vec.iter().map(|x| x * 2).collect();
//     // doubled is [2, 4, 6]

//     // 闭包
//     let x = 10;

//     let closure = |y| x + y;

//     let answer = closure(2); // answer is 12
// }

// 纯函数
fn test5() {
    // 纯函数
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // 非纯函数 — 依赖于外部可变状态
    static mut COUNTER: i32 = 0;

    fn increment_counter() {
        unsafe {
            COUNTER += 1;
        }
    }
}

// 递归函数
fn count_down(n: i32) {
    if n == 0 {
        // base case
        println!("Done!");
    } else {
        // recursive case
        println!("{}", n);
        count_down(n - 1); // function calls itself
    }
}
fn main() {
    count_down(5);
}

// 高阶函数
fn apply<F>(f: F) -> F {
    f
}

// fn compose<A, B, C>(f: fn(A) -> B, g: fn(B) -> C) -> fn(A) -> C {
//     move |x| g(f(x))
// }

// fn twice<F>(f: F) -> F
// where
//     F: FnOnce() -> (),
// {
//     move || {
//         f();
//         f();
//     }
// }

fn add_5(x: i32) -> i32 {
    x + 5
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn apply_example() {
    let f = apply(add_5);
    let result = f(10);
}

// fn compose_example() {
//     let h = compose(add_5, multiply);
//     let result = h(2, 3);
// }

// fn twice_example() {
//     let f = twice(|| println!("Hello!"));
//     f();
// }

fn test6() {
    let nums = vec![1, 2, 3, 4, 5];

    // Map
    let doubles = nums.iter().map(|x| x * 2);
    // [2, 4, 6, 8, 10]

    // Filter
    let evens = nums.iter().filter(|&x| x % 2 == 0);
    // [2, 4]

    // Fold
    let sum = nums.iter().fold(0, |a, b| a + b);
    // 15
}
