/*
枚举结构体

结构体给予你将字段和数据聚合在一起的方法，像 Rectangle 结构体有 width 和 height 两个字段。而枚举给予你将一个值成为一个集合之一的方法。比如，我们想让 Rectangle 是一些形状的集合，包含 Circle 和 Triangle 。为了做到这个，Rust 提供了枚举类型。

让我们看看一个需要诉诸于代码的场景，来考虑为何此时使用枚举更为合适且实用。假设我们要处理 IP 地址。目前被广泛使用的两个主要 IP 标准：IPv4（version four）和 IPv6（version six）。这是我们的程序可能会遇到的所有可能的 IP 地址类型：所以可以 枚举 出所有可能的值，这也正是此枚举名字的由来。

任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。IP 地址的这个特性使得枚举数据结构非常适合这个场景，因为枚举值只可能是其中一个成员。IPv4 和 IPv6 从根本上讲仍是 IP 地址，所以当代码在处理适用于任何类型的 IP 地址的场景时应该把它们当作相同的类型。

可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：
*/

pub fn enums() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    // 分别导出enum的两个实例
    // 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 使用成员调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    fn route(ip_kind: IpAddrKind) {
        println!("{:#?}", ip_kind)
    }
}

// 枚举字段，赋予类型

pub fn enums1() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}", home);

    // * 直接构建元组
    enum IpAddr1 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr1::V4(127, 0, 0, 1);
    let loopback = IpAddr1::V6(String::from("::1"));

    // todo 结构体，枚举
    struct Ipv4Addr {
        // --snip--
        b: i32,
    }

    struct Ipv6Addr {
        // --snip--
        a: String,
    }

    enum IpAddr2 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // ?枚举类型
    enum Message {
        // 类单元结构体
        Quit,
        // 普通结构体
        Move { x: i32, y: i32 },
        // 元组结构体
        Write(String),
        // 元组结构体
        ChangeColor(i32, i32, i32),
    }

    // 枚举也是一种结构
    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

pub fn enums2() {
    enum Option<T> {
        // 空值
        None,
        // 任意数据，泛型
        Some(T),
    }

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = Option::None;
}

pub fn enums3() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    // 返回1
    println!("{}", value_in_cents(Coin::Penny));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

pub fn enums4() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            // 要求必须穷尽所有的可能，包括none
            None => None,
        }
    }

    // todo other通配符
    let dice_roll = 9;
    match dice_roll {
        3 => 3,
        7 => 7,
        // other 可以获取match传递下来的值
        other => other,
        // 不要match传递下来的值
        // _ => reroll(),
    };

    // if let 简单判断, 语法糖
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // 语法糖2
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    // 语法糖3
    let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }
}
