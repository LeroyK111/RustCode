pub fn timer() {
    let x = 5; // ----------+-- 'b
               //           |
    let r = &x; // --+-- 'a  |
                //   |       |
    // println!("r: {}", r); //   |       |
                          // --+       |
                          // ----------+

    // !函数生命周期
    let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // println!("The longest string is {}", result);
    }

    // !结构体声明周期
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
}

// &i32        // 引用
// &'a i32     // 带有显式生命周期的引用
// &'a mut i32 // 带有显式生命周期的可变引用

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 编译失败是因为有定义域不同
// fn longest1(x: &str, y: &str) -> str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// 首先，这里有一个方法 level。其唯一的参数是 self 的引用，而且返回值只是一个 i32，并不引用任何值：
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// 适用于第三条生命周期省略规则的例子
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

let s: &'static str = "I have a static lifetime.";

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}