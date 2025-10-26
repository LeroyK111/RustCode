/*
在Rust中使用枚举时，Strum crate有助于摆脱样板代码，该功能是通过派生宏实现的。例如：
1，strum::Display：为enum实现std::fmt::Display，因此也实现了to_string() -> String方法。
2，strum::AsRefStr：实现AsRef<&static str>，因此，它不需要像使用to_string()那样分配内存。
3，strum::IntoStaticStr：实现“From<MyEnum> for &'static str”，工作原理类似于上一个选项。
4，strum::EnumString：实现std::str::FromStr和std::convert::TryFrom<&str>，允许将字符串转换为enum实例。
5，strum::EnumCount：添加常量COUNT: usize，值为枚举变量的数量。
6，strum::EnumIter：在枚举变量上实现迭代器，变量中的数据将被设置为Default:: Default()。
*/

#[derive(
    Debug,
    PartialEq,
    strum::Display,
    strum::IntoStaticStr,
    strum::AsRefStr,
    strum::EnumString,
    strum::EnumCount,
    strum::EnumIter,
)]
enum Color {
    Red,
    Blue(usize),
    Green { range: usize },
}

fn main() {
    assert_eq!(Color::Blue(2).to_string(), "Blue");
    assert_eq!(Color::Green { range: 5 }.as_ref(), "Green");
    assert_eq!(<&str>::from(Color::Red), "Red");

    assert_eq!(Color::Red, Color::from_str("Red").unwrap());
    assert_eq!(Color::COUNT, 3);
    assert_eq!(
        Color::iter().collect::<Vec<_>>(),
        vec![Color::Red, Color::Blue(0), Color::Green { range: 0 }]
    );
}

// 此外，Strum的不同宏支持行为定制。例如，可以使用属性#[strum(serialize = "redred")]更改将被转换为enum实例的字符串。
