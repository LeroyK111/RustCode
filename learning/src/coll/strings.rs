pub fn strMain() {
    // 空字符串
    let mut s = String::new();

    // 不可变字符串
    let data = "initial contents";

    // 转字符串
    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    // 创建字符串
    let s = String::from("initial contents");

    // 追加字符串 的 几种方式
    let mut s = String::from("foo");
    s.push_str("bar");
    s += "baz";
    s.push('l');
    println!("{}", s);

    // 使用format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // !索引字符串？根本不支持的。
    // * 1.字符串长度在内存中不是字面量的长度
    // * 2.let hello = "Здравствуйте";let answer = &hello[0];
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // 遍历字符串
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    /*
    标准库提供了很多围绕 String 和 &str 构建的功能，来帮助我们正确处理这些复杂场景。请务必查看这些使用方法的文档，例如 contains 来搜索一个字符串，和 replace 将字符串的一部分替换为另一个字符串。

    称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 或字符串 slice &str 类型，而不特指其中某一个。
    */
}
