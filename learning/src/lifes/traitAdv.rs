use std::ops::Add;

/*
Add其实是 左右相加的操作符号
*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<Rhs = Self> {
impl Add for Point {
    // 输出类型是
    type Output = Point;

    // 符号数的一种用法
    // + 的具体行为，由 Add::add 的实现定义
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn test() {
    let first = Point { x: 1, y: 0 };
    let second = Point { x: 2, y: 3 };

    let result = first + second;

    println!("{result:?}");
}

#[derive(Debug)]
struct Millimeters(u32);

struct Meters(u32);

// 为 Millimeters 添加 专属的 add + 方法 并且 add的右侧 是 Meters 类型
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

pub fn test2() {
    let result = Millimeters(500) + Meters(2);

    println!("{result:?}");
}

// 方法申明
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("人：挥动手臂");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("飞行员：驾驶飞机");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("巫师：施法飞行");
    }
}

pub fn test3() {
    /*
        一个类型可以实现多个 同名方法的 trait
    */
    let person = Human;

    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn test4() {
    println!("{}", Dog::baby_name());
    // 调用 Dog 实现 Animal 时定义的那个函数。
    println!("{}", <Dog as Animal>::baby_name());
}

use std::fmt::{self, Display};

trait OutlinePrint: fmt::Display {
    fn outline(&self) {
        let text = self.to_string();
        let len = text.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {text} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point2 {}

pub fn test5() {
    /*
        Display 的实现
        → 决定 Point 怎样显示成 "(1, 3)"

    OutlinePrint 的默认方法
        → 在显示结果外面加框

    impl OutlinePrint for Point {}
        → 选择使用默认方法
        */
    let point = Point2 { x: 1, y: 3 };

    println!("{}", point);
    point.outline();
}

// 元组(第一个元素是集合)
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里其实是 重写了 这个方法.
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn test6() {
    let words = vec![String::from("hello"), String::from("world")];

    let wrapper = Wrapper(words);

    println!("{wrapper}");
}

/*
泛型参数 Meters
    → 确定右操作数的类型

实现类型 Millimeters
    → 确定左操作数的类型

关联类型 Output
    → 确定结果类型
*/
