pub fn scope() {
    // 函数作用域
    /*
        当 s 进入作用域 时，它就是有效的。
        这一直持续到它 离开作用域 为止。
    */

    let s = "函数作用域字符串";
    println!("{}", s)
}

pub fn scope1() {
    let mut s = String::from("前\n");

    s.push_str("追加字符串"); // push_str() 在字符串后追加字面值

    println!("{}", s);

    /*
    这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。
    */
}
