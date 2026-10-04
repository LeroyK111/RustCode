/*
  这里就是ts or py 的接口概念了

  | 写法                                | 含义                    |
| --------------------------------- | --------------------- |
| impl Dog { ... }                  | 给 Dog 定义自己的方法         |
| impl Speak for Dog { ... }        | 为 Dog 实现 Speak 能力     |
| impl<T> Point<T> { ... }          | 给所有 Point<T> 定义通用方法   |
| impl Point<f32> { ... }           | 只给 Point<f32> 定义专用方法  |
| impl<T: Speak> Wrapper<T> { ... } | 只在 T 实现 Speak 时提供这些方法 |


  */

// 1. 定义能力：实现 Speak 的类型必须会叫
// *跨模块调用 trait 方法时，通常需要把 trait 也引入作用域，use animals::{Dog, Speak};
// *trait 或被实现的类型，至少有一个属于自己的 crate；不能随意给“别人定义的类型”实现“别人定义的 trait”。kaisery.github
pub trait Speak {
    fn speak(&self) -> String;

    // 默认方法：实现者可以直接使用，也可以重写
    fn introduce(&self) -> String {
        format!("我的叫声是：{}\n", self.speak())
    }
}

// 2. 定义两种不同的数据类型
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{}：汪汪！", self.name)
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{}：喵喵！", self.name)
    }
}

// 5. 不关心是猫还是狗，只要求它会叫
fn listen(animal: &impl Speak) {
    println!("{}", animal.introduce());
}
fn listen2<T: Speak>(animal: &T) {
    println!("{}", animal.speak());
}

// 可以是不同类型：狗和猫都可以
fn listenTwo(a: &impl Speak, b: &impl Speak) {
    println!("{}", a.speak());
    println!("{}", b.speak());
}

// 必须是同一个具体类型：两只狗，或两只猫
fn listenSame<T: Speak>(a: &T, b: &T) {
    println!("{}", a.speak());
    println!("{}", b.speak());
}

use std::fmt::Display;

// 多个能力用 +，不是“任选一个”，而是“全部满足”：
fn show<T: Speak + Display>(animal: &T) {
    println!("对象：{animal}");
    println!("叫声：{}", animal.speak());
}

// 另一种写法
fn show2<T>(animal: &T)
where
    T: Speak + Display,
{
    println!("对象：{animal}");
    println!("叫声：{}", animal.speak());
}

fn createAnimal() -> impl Speak {
    Dog {
        name: String::from("灰灰"),
    }
}

pub fn test() {
    /*
        trait Speak { /* 定义能力 */ }

    impl Speak for Dog { /* 给类型实现能力 */ }

    fn listen<T: Speak>(animal: &T) { /* 要求参数具备能力 */ }
        */
    let dog = Dog {
        name: String::from("小黑"),
    };

    let cat = Cat {
        name: String::from("小白"),
    };

    listen(&dog);
    listen2(&cat);

    let animal = createAnimal();
    println!("{}", animal.speak());
}
