fn recursion() {
    /*
        | 场景             | 要解决的问题           | Box 的作用             |
    | -------------- | ---------------- | ------------------- |
    | 递归结构等需要间接存储的场景 | 不能直接内嵌一个大小无法确定的值 | 用大小已知的指针表示连接数据      |
    | 移动大量内嵌数据       | 不希望移动所有权时搬动整份数据  | 移动 Box，堆上数据留在原位置    |
    | Box<dyn Trait> | 想拥有数据，但通过共同接口使用它 | 拥有具体值，向外暴露 trait 接口 |
        */

    #[derive(Debug)]
    enum List {
        // 发生了无限递归, ts可以这么做哦
        // Cons(i32, List),
        Cons(i32, Box<List>),
        Nil,
    }
    // 这里只是为了 使用 cons 和 nil 再次申明
    use List::{Cons, Nil};

    // 先构造结尾
    let end = Nil;

    // 再从后向前，逐个接上节点
    let third = Cons(3, Box::new(end));
    let second = Cons(2, Box::new(third));
    let first = Cons(1, Box::new(second));

    println!("{first:?}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}")
}

pub fn test() {
    // 创建一个堆
    // let b = Box::new(5);
    /*
    Box::new(5)：创建管理这个堆上值的 Box。

    b：拥有那份数据，不只是借用它。

    *b：通过解引用访问里面的值。

    b 被销毁时：清理里面的数据，并释放相应的堆内存。
        */
    // println!("b = {b}"); // b = 5
    // println!("{}", *b); // 5

    // !实际用法
    // recursion();
}
