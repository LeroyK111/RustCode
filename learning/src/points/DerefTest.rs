use std::ops::Deref;

// 元组结构体：包装一个 T 类型的值
// 注意：这个教学用 MyBox 不会把内部值分配到堆上

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // 创建 MyBox，把传入的值放进第一个字段
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

// 实现 Deref，让 MyBox 支持使用 * 访问内部值
impl<T> Deref for MyBox<T> {
    // 解引用的目标类型是 T
    type Target = T;

    // 返回内部值的引用，不转移内部值的所有权
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 函数需要的是字符串切片引用 &str
fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn test() {
    // 1. 普通引用：使用 * 访问引用指向的值
    let x = 5;
    let reference = &x;

    assert_eq!(5, *reference);
    // println!("普通引用解引用：{}", *reference);

    // 2. 标准库 Box：同样可以使用 *
    let boxed = Box::new(5);

    assert_eq!(5, *boxed);
    // println!("Box 解引用：{}", *boxed);

    // 3. 自定义 MyBox：实现 Deref 后也可以使用 *
    let my_box = MyBox::new(5);

    // 如果使用 derfe 则自定义结构体的值 不能指向value
    // println!("{}", *my_box);
    // println!("{}", my_box.0);

    // assert_eq!(5, *my_box);
    // println!("MyBox 解引用：{}", *my_box);

    // 4. 手动调用 deref：返回的是内部值的引用
    // let inner_reference: &i32 = my_box.deref();

    // assert_eq!(5, *inner_reference);
    // println!("手动 deref 后解引用：{}", *inner_reference);

    // 5. Deref 强制转换：自动转换引用以匹配函数参数
    // let name = MyBox::new(String::from("Rust"));

    // &MyBox<String> → &String → &str
    // hello(&name);

    // 教程中的手动写法，得到同样的结果
    // hello(&(*name)[..]);
}
