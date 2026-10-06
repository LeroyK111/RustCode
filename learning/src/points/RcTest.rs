/*
怎样让多个地方共同拥有同一份数据，而不是把数据移动给其中一个，或者复制出多份数据。 Rc<T> 通过记录共享所有者的数量，让数据一直保留到最后一个所有者消失。

Box：把这份数据交给一个所有者
Rc ：让多个所有者共同管理这份数据
*/

use std::rc::Rc;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// fn err() {
//     use List::{Cons, Nil};
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));

//     let b = Cons(3, Box::new(a));
// a的所有权已经给b了
// let c = Cons(4, Box::new(a)); // 编译错误
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn test() {
    // a 指向共享的列表：5 → 10 → Nil
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("创建 a 后：{}", Rc::strong_count(&a));

    // b：3 → 共享列表
    // Rc::clone：增加一个所有者
    let b = Cons(3, Rc::clone(&a));

    println!("创建 b 后：{}", Rc::strong_count(&a));

    {
        // c：4 → 同一份共享列表
        let c = Cons(4, Rc::clone(&a));

        println!("创建 c 后：{}", Rc::strong_count(&a));

        // 内部作用域结束，c 被销毁
    }

    // ! Rc<T> 用于单线程场景。
    // ! 多个共享所有者不能直接借此获得同时修改数据的能力；教程下一节才介绍与 RefCell<T> 配合的方式。
    //  rc只能用来计数和保存对象

    println!("c 销毁后：{}", Rc::strong_count(&a));

    // main 结束，先销毁 b，再销毁 a
}
