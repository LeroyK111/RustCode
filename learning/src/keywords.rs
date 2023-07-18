pub fn test_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// 常量, 无法变更
pub fn constants() {
    const THREE_HOURS_INSECONDS: u32 = 60 * 60 * 3;
}

// 隐藏，当定义域不同时，同一个变量改变时，会发生不同的结果。
pub fn Shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
