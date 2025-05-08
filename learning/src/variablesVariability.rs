// 变量mut
pub fn test_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// 常量, 无法变更，const
pub fn constants() {
    const THREE_HOURS_INSECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_INSECONDS)
}

// 隐藏，当定义域不同时，同一个变量改变时，会发生不同的结果。
pub fn Shadowing() {
    let x: i32 = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// 变量类型的强制性
pub fn testType() {
    // todo 这种可以修改已命名的值
    let spaces = "   ";
    // println!("{spaces}?");
    // 这里是遮盖了
    let spaces = spaces.len();
    // println!("{spaces}");
    
    // todo 这种就会报错，虽然你可以修改value，但不允许修改它的type。
    // let mut spaces = "   ";
    // 但是你加let const 等等，就可以新建变量
    // spaces = spaces.len();
    // println!("{spaces}");
}
