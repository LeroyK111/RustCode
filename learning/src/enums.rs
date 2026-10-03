/*
枚举结构体

结构体给予你将字段和数据聚合在一起的方法，像 Rectangle 结构体有 width 和 height 两个字段。而枚举给予你将一个值成为一个集合之一的方法。比如，我们想让 Rectangle 是一些形状的集合，包含 Circle 和 Triangle 。为了做到这个，Rust 提供了枚举类型。

让我们看看一个需要诉诸于代码的场景，来考虑为何此时使用枚举更为合适且实用。假设我们要处理 IP 地址。目前被广泛使用的两个主要 IP 标准：IPv4（version four）和 IPv6（version six）。这是我们的程序可能会遇到的所有可能的 IP 地址类型：所以可以 枚举 出所有可能的值，这也正是此枚举名字的由来。

任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。IP 地址的这个特性使得枚举数据结构非常适合这个场景，因为枚举值只可能是其中一个成员。IPv4 和 IPv6 从根本上讲仍是 IP 地址，所以当代码在处理适用于任何类型的 IP 地址的场景时应该把它们当作相同的类型。

可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：
*/

use serde_json::de;

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

    fn route(ip_kind: IpAddrKind) {
        println!("{:#?}", ip_kind)
    }

    // 使用成员调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 结构体 + 枚举组合
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    // 枚举实例化
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// 枚举字段，赋予类型

pub fn enums1() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
        // 报错是枚举对象没有真正被实例化
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

    #[derive(Debug)]
    struct Ipv4Addr {
        b: i32,
    }

    #[derive(Debug)]
    struct Ipv6Addr {
        a: String,
    }

    #[derive(Debug)]
    enum IpAddr2 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let ip4 = IpAddr2::V4(Ipv4Addr { b: 123 });

    let ip6 = IpAddr2::V6(Ipv6Addr {
        a: String::from("::1"),
    });

    println!("{:#?}", ip4);
    println!("{:#?}", ip6);

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
    /*
    Rust 没有 null、none 或 None 这样的空值关键字.
    但是标准库中有
    let number: Option<i32> = std::option::Option::None;
    */
    let no = Option::None;
    let no1: Option<i32> = std::option::Option::None;

    #[derive(Debug)]
    enum Option1<T> {
        // 空值
        None,
        // 任意数据，泛型
        Some(T),
    }

    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_char = Some('e');
    println!("{:?}", some_char);
    let absent_number: Option<i32> = no;

    let absent_number2: Option<i32> = Option::None;

    println!("{:?}", absent_number);
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
            // 几乎是必须的
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 还是类
    println!("enums3: {:?}, {:?}, {:?}", five, six, none);

    #[derive(Debug)] // 这样可以立刻看到州的名称
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    println!(
        "enums3: {:?}, {:?}",
        value_in_cents2(Coin2::Penny),
        value_in_cents2(Coin2::Quarter(UsState::Alabama)),
    );

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // 缺省值
        _ => reroll(),
    }

    // 同样支持直接这么写
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
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

    // Quarter 携带的州信息
    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    // 硬币类型
    #[allow(dead_code)] // 不读
    #[derive(Debug)] // 结构可打印
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // 创建一个具体的 Coin 值
    let coin = Coin::Quarter(UsState::Alaska);

    // 写法一：match
    let mut count = 0;

    match &coin {
        // 要么符合结构体/要么符合值
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
        }
        _ => count += 1,
    }

    println!("match count = {count}");

    // 写法二：if let + else
    // 重新初始化 count，独立演示第二种写法
    let mut count = 0;

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("if let count = {count}");

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
                // -- snip --
            }
        }
    }



    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is a quarter from a state that existed in 1900."))

        } else {
            Some(format!("{state:?} is a quarter from a state that did not exist in 1900."))
        }
    }

    let coin2 = Coin::Quarter(UsState::Alaska);
    println!("检查硬币是否存在: {:?}", describe_state_quarter(coin2));

}
