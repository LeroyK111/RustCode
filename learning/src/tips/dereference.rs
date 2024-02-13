/*
! Rust中的强制解引用

智能指针在Rust的所有权系统中扮演着关键的角色，提供了比传统指针更多的功能。与C语言中的指针主要用于引用内存位置不同，Rust的智能指针，如Box<T>、Rc<T>和RefCell<T>，管理它们所持有数据的生命周期。这种管理包括自动内存释放和引用计数等功能，在不牺牲效率的情况下确保内存安全。

让我们考虑最简单的智能指针：Box<T>。Box<T>使你能够将值存储在堆上，这样就可以有效地移动值以防止深度复制。移动Box<T>意味着只复制指针，而不复制潜在的值。让我们检查一下Rust如何像对普通指针那样对Box<T>解引用。


尽管Box<T>本身是一个轻量级值，包含有关其堆存储数据的元数据，但它模仿了一个常规指针，因为它可以使用解引用操作符(*)对其解引用，以访问其保存的值。这就是为什么Box<T>等类型被称为智能指针的原因。
*/




fn demo() {
    // 值在栈中
    let x = 10;
    // 指向x的常规指针
    let x_ptr = &x;
    // 解引用指针
    println!("Value using regular pointer: {}", *x_ptr); 

    // 指向堆上整数的智能指针(Box)
    let x = Box::new(10);
    // 自动的对潜在的值进行解引用
    println!("Value using smart pointer: {}", *x); 
}




/*
todo 自定义智能指针
*/


// 让我们尝试创建一个类似Box<T>的智能指针。为简单起见，底层值将存储在栈中而不是堆中。

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref trait由标准库提供，在一个类型上实现这个trait将允许该类型被解引用。让我们为MyBox<T>实现它：

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
deref方法的签名表明，它接受MyBox<T>的不可变引用，并返回对底层值类型T的的引用。

测试代码如下：
*/

fn main() {
    let x = MyBox::new(10);
    println!("Value using our smart pointer: {}", *x); 
}


// 那么到底发生了什么？deref方法使编译器能够接受实现Deref trait的任何类型，并调用deref方法来获取一个它知道如何解引用的&引用。

// 所以当我们对x解引用时，Rust在后台运行：*(x.deref())

// 初始解引用操作符被替换为对deref()的调用，以返回一个引用，然后最终使用(*)对原始值进行解引用。


/*
隐式强制解引用

现在让我们来看看强制解引用是如何让我们节省一些思考并防止指针语法混乱的。
fn main() {
    let x = Box::new(String::from("some string literal"));
}
x是一个Box<String>，它只是在堆上存储一个字符串字面值。假设我们有一个函数，它接受一个字符串字面值，并可以对它做任何事情。
fn take_str(str: &str) {
    // does whatever
}
如何从Box<String>中获取&str？以下都是等价的。
fn main() {
    let x = MyBox::new(10);
    println!("Value using our smart pointer: {}", *x); // prints correctly

    let x = Box::new(String::from("some string literal"));

    // Box -> String -> str -> &str
    take_str(&(*(*x)));

    // Box -> String 然后取一个字符串切片来获取&str
    take_str(&(*x)[..]);

    // 在Box上调用deref，然后在String上调用
    take_str(x.deref().deref());

    // 执行与上述等价的强制解引用
    take_str(&x);
}
注意String也实现了Deref。所以我们可以对它调用deref()。但我们感兴趣的是为什么take_str(&x)也可以正常工作。

当为所涉及的类型定义了Deref trait时，Rust将分析这些类型，并根据需要多次使用Deref::deref来获取与参数类型匹配的引用。

这意味着我们还可以做以下事情：
fn take_string(string: &String) {
    // Do whatever
}

fn take_str(str: &str) {
    // Do whatever
}

fn main() {
    let x = Box::new(String::from("some string literal"));

    // Rust将只调用一次解引用
    take_string(&x); 

    // Rust将调用两次解引用
    take_str(&x); 
}
所有这些转换都在编译时解决，因此强制解引用不会导致运行时性能损失！
*/