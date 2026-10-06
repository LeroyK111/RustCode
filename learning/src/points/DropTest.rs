struct CustomSmartPointer {
    data: String,
}

// 定义这个类型被销毁时，要执行的代码
impl Drop for CustomSmartPointer {
    // 定义方法写 fn drop，提前销毁写 drop(c)，不要写 c.drop()。
    fn drop(&mut self) {
        // 生命周期 销毁指针
        println!("正在清理：{}", self.data);
    }
}

pub fn test() {
    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let b = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("两个对象已经创建");

    // 没有手动清理: 本质也是调用drop
    // main 的作用域结束时，Rust 会自动清理 d 和 c

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("对象已经创建");

    // 把 c 交给标准库函数，立即销毁它
    drop(c);

    println!("对象已清理，但 main 还没有结束");

    // 不能再使用 c，因为它已经被移动并销毁
    // println!("{}", c.data);
}
