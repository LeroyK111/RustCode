pub fn call() {
    // 创建存储列表，数据结构
    // let mut v: Vec<i32> = Vec::new();
    // 简写方式
    // let v = vec![1, 2, 3];
    let mut v = vec![];
    // 开始堆栈
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // 引用
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // 获取
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // println!("{}", does_not_exist);

    // get 可以避免索引报错。取不到的值，用none代替
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);
}

pub fn call2() {
    // 已经不会报错了
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
}

pub fn call3() {
    let v = vec![100, 32, 57];
    // 循环引用
    for i in &v {
        println!("{i}");
    }

    // 循环改写
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 枚举结构自适应
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
