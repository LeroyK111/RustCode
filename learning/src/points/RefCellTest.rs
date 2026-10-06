/*
RefCell<T> 允许你通过不可变引用修改内部数据，但内部借用仍必须遵守规则，只是把检查放到运行时。 与上一节连接起来：Rc 解决“多人共同拥有”，RefCell 解决“通过共享访问修改内部值”。
*/

use std::cell::RefCell;

use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    // 保存引用了, 则需要保护生命周期
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub fn handleMsg() {
    // 不实际发送消息，只记录收到的消息
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // 虽然是 &self，仍能通过 RefCell 修改内部 Vec
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        // 借用参数, 入参100
        let mut tracker = LimitTracker::new(&mock_messenger, 100);

        tracker.set_value(80);

        // borrow()：只读地查看记录结果
        let msg = mock_messenger.sent_messages.borrow();
        assert_eq!(msg.len(), 1,);
        println!("{msg:?}")
    }

    it_sends_an_over_75_percent_warning_message()
}

pub fn test() {
    // let x = 5;
    // let y = &mut x; // 编译错误：x 没有声明为 mut

    // 没有写 let mut
    let value = RefCell::new(5);

    {
        /*
                | 方法           | 返回类型      | 能做什么     |
        | ------------ | --------- | -------- |
        | borrow()     | Ref<T>    | 只读访问内部值  |
        | borrow_mut() | RefMut<T> | 读取和修改内部值 |
                */

        // 申请对内部数字的可变借用
        let mut number = value.borrow_mut();

        *number += 10;
    } // number 被销毁，这次可变借用结束

    println!("{:?}", value);
    // 申请只读借用
    let number = value.borrow();
    println!("{}", *number); // 15

    // 销毁 borrow 的只读守护
    drop(number);
    let mut number = value.borrow_mut();
    *number -= 10;
    println!("{}", *number); // 5
}

pub fn handlRcAndRefcell() {
    /*
    b：3 ──┐
           ├──→ a：5 → Nil
    c：4 ──┘       ↑
                   │
                 value
        */
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 创建一个可以共享、可以内部修改的数字
    let value = Rc::new(RefCell::new(5));

    // a 的数字字段和 value 共享同一份 RefCell
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // b 和 c 共享列表 a
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // 修改共享的那一个数字：5 → 15
    *value.borrow_mut() += 10;
    let resutl: std::cell::Ref<'_, i32> = value.borrow();

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
    println!("result = {resutl}");
}
