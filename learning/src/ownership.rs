pub fn scope() {
    // 函数作用域
    /*
        当 s 进入作用域 时，它就是有效的。
        这一直持续到它 离开作用域 为止。
    */
    let s = "函数外作用域";
    {
        // 内部属性和方法，只能内部调用，除非你给返回值
        let s = "函数作用域字符串";
    }

    /*
    这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。
    */

    println!("{}", s)
}

pub fn scope1() {
    // *使用from函数基于字符串的字面值来创建string
    // *:这两个冒号 :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。
    let mut s = String::from("前\n");

    s.push_str("追加字符串"); // push_str() 在字符串后追加字面值

    println!("{}", s);

    /*
    ?就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

    */
}

pub fn scope2() {
    // 变量和数据交互的方式：这个会让rust深复制。
    let x = 5;
    let y = x;

    // 方式二：这种则会让rust潜复制，让s2指向s1点内存地址。
    let s1 = String::from("hello");
    let s2 = s1;

    /*
    todo 当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

    todo rust的解决思路：s1被清理，s2彻底取代s1。
    */

    // s2正常使用
    // println!("{s2}");
    // s1 则报错error[E0382]: borrow of moved value: `s1`
    // println!("{s1}")
}

pub fn scope3() {
    // 深复制，只针对字面量from
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // 拷贝, 这里没有使用字面量from，
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

pub fn scope4() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
                   // 但 i32 是 Copy 的，
                   // 所以在后面可继续使用 x

    let response = gives_ownership();
    println!("{response}");

    // jie解构赋值
    let str1 = String::from("abc");
    let (s2, s2lenght) = calculate_length(str1);
    println!("{s2}, {s2lenght}")
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
                
}

// 返回一个元祖
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}