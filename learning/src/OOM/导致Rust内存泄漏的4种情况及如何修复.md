# 导致Rust内存泄漏的4种情况及如何修复

Rust的内置所有权模型和编译时检查降低了内存泄漏的可能性和风险，但它们仍然很有可能发生。
内存泄漏不违反所有权规则，因此借用检查器允许它们在编译时可以编译通过。内存泄漏是低效的，通常不是一个好主意，特别是在有资源限制的情况下。
另一方面，如果将不安全行为嵌入到unsafe块中，它也会编译通过。在这种情况下，无论操作是什么，内存安全都是你的责任，例如指针解引用、手动内存分配或并发问题。

## 所有权和借用导致的内存泄漏

借用检查器在编译器执行程序之前可以防止悬空引用、use-after-free错误和编译时的数据竞争。但是，在分配内存时，如果没有在整个执行过程中删除内存，则可能发生内存泄漏。下面是如何实现双重链表的一个例子。程序可以成功运行，但会出现内存泄漏问题：

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
        prev: None,
    }));

    let second = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::clone(&first)),
        prev: Some(Rc::clone(&first)),
    }));

    first.borrow_mut().next = Some(Rc::clone(&second));
    first.borrow_mut().prev = Some(Rc::clone(&second));

    println!("Reference count of first: {}", Rc::strong_count(&first)); 
    println!("Reference count of second: {}", Rc::strong_count(&second)); 

}
```

这个程序的问题发生在两个节点之间的循环引用中，导致内存泄漏。由于RC智能指针默认情况下不处理循环引用，因此每个节点都持有对另一个节点的强引用，从而导致了循环引用。
在main函数执行之后，second和first变量的引用计数将等于first的值，尽管它不再可访问。这将导致内存泄漏，因为没有任何节点被释放：

```sh
Reference count of first: 3
Reference count of second: 3
```

可以通过以下方式修复这样的情况：
对一个链路方向使用弱引用，如weak<'T>
在函数结束前手动打破循环

下面是在prev字段上使用弱指针来解决这个问题的例子：

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
        prev: None,
    }));

    let second = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::clone(&first)),
        prev: Some(Rc::downgrade(&first)),
    }));

    first.borrow_mut().next = Some(Rc::clone(&second));
    first.borrow_mut().prev = Some(Rc::downgrade(&second));

    println!("Reference count of first: {}", Rc::strong_count(&first)); 
    println!("Reference count of second: {}", Rc::strong_count(&second)); 

    println!("First value: {}", first.borrow().value);
    println!("Second value: {}", second.borrow().value);

    let next_of_first = first.borrow().next.as_ref().map(|r| r.borrow().value);
    println!("Next of first: {}", next_of_first.unwrap());

    let prev_of_second = second.borrow().prev.as_ref().unwrap().upgrade().unwrap();
    println!("Prev of second: {}", prev_of_second.borrow().value);
}
```
<!-- 可以使用Weak<RefCell<Node>>来防止内存泄漏，因为弱引用不会增加强引用计数，并且节点可以被释放。执行结果如下： -->
```sh
Reference count of first: 2
Reference count of second: 2
First value: 1
Second value: 2
Next of first: 2
Prev of second: 1
```

## std::mem::forget函数

在必要时，可以有意地使用std::mem::forget函数来泄漏Rust项目中的内存，编译器认为它是安全的。
即使没有回收内存，也不会有不安全的访问或内存问题。
std::mem::forget获取值的所有权，并且在不运行析构函数的情况下forget它，由于内存中保存的资源没有被释放，因此将存在内存泄漏：

```rust
use std::mem;

fn main() {
    let data = Box::new(42);
    mem::forget(data);
}

```
在运行时，Rust跳过通常的清理过程，数据变量的值不会被删除，并且为数据分配的内存在函数执行后泄漏。

## 使用unsafe块泄漏内存
在使用原始指针时，需要自己进行内存管理，这就有可能导致内存泄漏。以下是在unsafe块中使用原始指针可能导致内存泄漏的原因：
```rust
fn main() {
    let x = Box::new(42);
    let raw = Box::into_raw(x); 

    unsafe {
        println!("Memory is now leaked: {}", *raw);
    }
}
```
在这种情况下，内存没有显式释放，并且在运行时将存在内存泄漏。在程序执行结束之后，内存将被释放，内存使用效率较低。


## 故意用Box::leak泄漏内存

Box::leak函数可以故意泄漏内存，当需要在整个运行时使用一个值时，这种方式是正确的：
```rust
fn main() {
    let x = Box::new(String::from("Hello, world!"));
    let leaked_str: &'static str = Box::leak(x);
    println!("Leaked string: {}", leaked_str);
}
```

不要滥用这种方式，如果你需要静态引用来满足特定的API需求，那么Box::leak是有用的。


## total 
```
修复Rust中的内存泄漏修复内存泄漏的黄金法则是从一开始就避免它们，除非你的用例需要这样做。遵循所有权规则是一个好主意。事实上，通过借用检查器，Rust实施了很好的内存管理实践：
1，当你需要在不转移所有权的情况下借用值时使用引用。
2，可以尝试使用Miri工具来检测未定义的行为并捕获与内存泄漏相关的错误。
3，在自定义类型上实现Drop trait以清理内存。
4，不要多余地使用std::mem::forget。检查Box<T>，以便在值超出范围时自动清理堆内存。
5，不要无缘无故地到处throw unsafe块。
6，使用Rc<T>或Arc<T>共享变量所有权。
7，对于内部可变性，使用RefCell<T>或Mutex<T>。如果需要确保安全的并发访问，它们很有帮助。遵循这些技巧应该可以处理Rust程序中的所有内存泄漏，以构建低内存需求的Rust程序。
```