use std::any::type_name_of_val;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn bug() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    /*
    1. a 的节点：5 → Nil
    2. Rc::clone(&a) clone 就是 指针放入

    b 的节点：10 → 5 → Nil
              ↑
           a 的节点

    a.tail() 指向 self Nil
    link 得到 self Nil

    !这就是 if let的逻辑
             Cons(5, RefCell::new(Rc::new(Nil)))
                    └──────────┬────────────┘
                            link 引用这里

    let result = a.tail();
    match result {
        Some(link) => {

            let mut tail_pointer = link.borrow_mut();
            let new_pointer = Rc::clone(&b);

            *tail_pointer = new_pointer;
        }
        None => {
            没有尾部字段，不执行操作
        }
    }

    ! 随后就是互相引用
    a 的下一项：
    原来 → Nil
    现在 → b 的节点

        ┌─────────────┐
        ↓             │
    a 的节点：5 → b 的节点：10

    5 → 10 → 5 → 10 → 5 → 10 → ……

    变量 a、b 离开作用域后，两边计数都只减到 1，因为节点内部还互相持有对方。计数不是 0，节点就不会被销毁。

    !其实就是rc + refcell rc塑造链式结构，refcell 可以修改nil的值，导致b填充进了 nil的位置，这就 单向循环链表

    */
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // 这里则发生了 无限递归
    // println!("a next item = {:?}", a.tail());
}

fn useOther() {
    let strong = Rc::new(5);
    // 建立弱引用, 指向rc
    let weak = Rc::downgrade(&strong);
    // 尝试取得 Rc
    let result = weak.upgrade();

    println!("{}", type_name_of_val(&result))
}

fn useOther1() {
    // a：第一个所有者
    let a = Rc::new(String::from("这是一份共享数据"));

    // b：第二个所有者，指向同一份数据
    let b = Rc::clone(&a);

    // weak：指向同一份数据，但不拥有它
    let weak = Rc::downgrade(&a);

    println!("强引用数量：{}", Rc::strong_count(&a)); // 2
    println!("弱引用数量：{}", Rc::weak_count(&a)); // 1

    // 销毁第一个所有者
    drop(a);

    // b 仍然拥有数据，所以数据还在
    if !b.is_empty() {
        println!("销毁 a 后，b 还能读取：{}", b);
        println!("剩余强引用数量：{}", Rc::strong_count(&b)); // 1
        println!("weak 查询：{:?}", weak.upgrade()); // Some(...)
    }

    // 销毁最后一个所有者
    drop(b);

    // weak 仍然存在，但无法让数据继续存在
    println!("销毁 b 后，weak 查询：{:?}", weak.upgrade()); // None
}

pub fn useWeak() {
    // 学习一下 weak和 rc的关系
    // useOther();
    useOther1();

    #[derive(Debug)]
    struct Node {
        value: i32,
        // 可以更换父节点，但不拥有父节点
        parent: RefCell<Weak<Node>>,
        // 拥有子节点，也可以修改子节点集合
        children: RefCell<Vec<Rc<Node>>>,
    }

    // 创建子节点，暂时没有父节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "1：leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // 说明leaf.parent 已经取不到rc了
    println!("2：leaf parent = {:?}", leaf.parent.borrow().upgrade(),);

    {
        // 创建父节点，让它拥有 leaf 的一个共享句柄
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        /*
        原来：
        leaf.parent → 空 Weak

        现在：
        leaf.parent ──Weak──→ branch 的节点


        所以真正避免互相拥有的，不是 RefCell，而是这里保存的连接类型从 Rc 换成了 Weak。RefCell 仍然只是负责允许修改连接。
        */

        // 子节点通过 Weak 记住父节点: leaf.parent 指向 rc
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // 一旦改变强引用指向. 则发生泄漏
        // leaf.children.borrow_mut().push(Rc::clone(&branch));

        // leaf.parent 指向父节点，因此它增加的是父节点的弱引用计数
        // 查看计数器
        println!(
            "3：branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // 查看子节点 计数器
        println!(
            "4：leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        // 尝试访问父节点
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("父节点的值：{}", parent.value);
        } // 临时获得的 parent Rc 在这里销毁
    } // branch 离开作用域，父节点被销毁

    println!("5：leaf parent = {:?}", leaf.parent.borrow().upgrade(),);

    println!(
        "5：leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
