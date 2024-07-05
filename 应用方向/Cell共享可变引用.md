# 深入研究Rust的内部可变性- Cell是如何工作的？

Cell 如何提供内部可变性？

基本上，Cell通过确保没有指向其保存的数据的指针并且在单线程环境中执行来实现这一点。

有了这些限制，更改Cell中的数据是完全可以的。想想看，如果我们知道Cell中没有指向数据的指针，并且它不是跨线程共享的，则可以保证我们对它具有独占访问权。

现在的问题是，Cell是如何施加这些约束的？Cell通过从不返回对其内部数据的引用来实现这一点，它总是返回数据的副本。因此，这已经告诉Cell适用于内存开销小的类型，例如整数。

此外，Cell没有实现Sync，因此它不能在线程边界之间共享。

Cell的构建块是UnsafeCell，这是Rust内部可变性的构建块之一。UnsafeCell允许我们在任何时候获得一个原始的独占指针，指向它所保存的数据。这当然是一个不安全的操作，所以我们必须在unsafe{}块中进行操作。

```rust
use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>
}

// 禁止跨线程使用Cell
impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(self, value: T) {
        // 用一个新值覆盖单元格所指向的值
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T where T: Copy {
        // 返回Cell所指向的数据的副本
        unsafe { *self.value.get() }
    }
}
```

这里我们使用UnsafeCell来存储Cell的数据，不允许在线程之间共享此类型，最后，我们从不引用Cell中的数据。注意，get方法只适用于实现Copy的类型，并且返回内部类型的副本。
