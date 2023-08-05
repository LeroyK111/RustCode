/*
! 这里放入 interfaces 接口
*/
pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn summarize_test(&self) -> String {
        format!("调用我自己函数 {}", self.summarize())
    }
}
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// 结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// 引入接口详细方法
impl Summary2 for NewsArticle {}

// * 约束了 机构体的方法，
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 引入接口模糊方法
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// todo 标准格式化 接口
use std::fmt::{Debug, Display, Error, Formatter, Result};
// 多接口使用
impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.reply)
    }
}

// *接受结构对象，跨作用域引用其他结构体的方法
// pub fn notify(item: &impl Summary) {
//     println!("为其他 {}", item.summarize())
// }

// Trait Bound 语法糖
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify(item1: &impl Summary, item2: &impl Summary2) {
//     println!("Breaking1 news! {}", item1.summarize());
//     println!("Breaking2 news! {}", item2.summarize());
// }

// pub fn notify<T: Summary, U: Summary2>(item1: &T, item2: &U) {
//         println!("Breaking1 news! {}", item1.summarize());
//         println!("Breaking2 news! {}", item2.summarize());
// }

// * 多接口使用
// pub fn notify(item: &(impl Summary + Display)) {
//     println!("Breaking news! {}", item.summarize());
//     println!("news! {}", item);
// }

// pub fn notify<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
//     println!("news! {}", item);
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//     println!("难受")
// }
// todo 推荐写法
// fn some_function1<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     println!("Breaking news! {}", item.summarize());
// }

// !在返回值上定义
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("??"),
        content: String::from("xx"),
        reply: false,
        retweet: false,
    }
}

pub fn interfaces() {
    // todo 引入模糊方法
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // println!("引入模糊方法: {}", tweet.summarize());
    // println!("引入模糊方法: {}", tweet.summarize_author());
    // println!("{}", tweet.summarize_test());

    // todo 引入详细方法
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    // println!("引入详细方法: {}", article.summarize());
    // *避开作用域问题，调用给其他函数引用
    // notify(&tweet);
    // notify(&tweet, &article);
    // some_function(&tweet, &article)
    // some_function1(&tweet, &article)
    // let D = returns_summarizable();
    // println!("{}", D.summarize());

    let a = Pair { x: 1, y: 1 };
    a.cmp_display();

    // 给泛型添加方法
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 给泛型实现方法
use std::string::ToString;

struct Circle {
    radius: i32,
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}
