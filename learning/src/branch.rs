/*
!值得注意的是代码中的条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。例如，尝试运行以下代码
*/
pub fn testIf() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn testIf_2() {
    let condition = true;
    // 语法糖, 但是返回值的类型必须一致
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub fn lp() {
    // 当运行这个程序时，我们会看到连续的反复打印 again!，直到我们手动停止程序。大部分终端都支持一个快捷键，ctrl-c，来终止一个陷入无限循环的程序。
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

//  循环嵌套
pub fn lp2() {
    let mut count = 0;
    // 加入标签，指示跳出的是哪层循环
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// while 条件循环
fn lp3() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// !for循环 最好用的循环
fn lp4() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // rev翻转字符串，1..4是一个数组的语法糖
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
