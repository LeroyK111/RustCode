# 与Rust编译器的斗争

由于Rust编译器对rust代码应用的了严格的检验规则，所以作为一个正在学习Rust的人，肯定会遇到过无数不得不与Rust编译器抗争的事件。这个系列文章通过一些实际案例来研究你可能遇到的各种错误，希望这些案例能给你提供帮助。

我们来看第一个案例，考虑下面的函数，它接收一个借用的Option<String</String作为输入，如果字符串为空，则打印为空。

```rust
fn parse_option(opt: Option<String) {
    println!("{}", opt.unwrap_or("empty".to_owned()));
}
```

这个简单的单行函数会导致编译错误：
```rust

error[E0507]: cannot move out of `*opt` which is behind a shared reference
 -- <source:2:20
  |
2 |     println!("{}", opt.unwrap_or("empty".to_owned()));
  |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ move occurs because `*opt` has type `Option<String`, which does not implement the `Copy` trait

error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
Compiler returned: 1
```
让我们看看发生了什么。首先，参数opt的类型是Option<String，也就是说，它是不可变借用的Option，这意味着我们既不能移出它，也不能修改它。现在，让我们看一下Option::unwrap_or()的方法签名。

```rust
pub fn unwrap_or(self, default: T) - T
```

该方法明确地将self作为第一个参数，这意味着它将在调用该方法时移动对象。这正是编译器所说的：不能移出共享引用背后的“*opt”。

现在我们知道了根本原因，让我们看看如何解决这个问题。最终，我们需要访问(读取)包含的值，但是我们可以在不改变opt的函数参数类型的情况下做到这一点吗？毕竟，这似乎是一个相当常见的场景，所以必须有一种方法来做到这一点。

如果你检查Option的文档，你会发现一个方法Option::as_deref()，它的签名如下：

```rust
pub fn as_deref(self) - Option<<T as Deref::Target
```
该方法基本上将(不修改/移动)Option<T或Option<T转换为Option<T，这正是我们想要的。该方法的第一个参数是self，因此我们可以在Option<T类型上调用该方法。从技术上讲，该方法将输出另一个Option实例，该实例具有对所包含值的引用，我们现在可以调用unwrap_or()方法。因此，下面是修复后的代码：

```rust
fn parse_option(opt: Option<String) {
    println!("{}", opt.as_deref().unwrap_or("empty"));
}
```
首先，我们在调用unwrap_or()方法之前调用as_deref()。第二，我们提供了另一个字符串empty作为str类型而不是string类型。希望这个例子能帮助你了解Rust编译器。

as_ref()和as_deref()看起来非常相似，那么as_ref()是否也可以工作。它们的关键区别在于as_ref()是字面引用，而as_deref()是对Deref::Target的引用。在本例中，该选项包含的值类型是String。在这种情况下，我们得到：

```rust
opt.as_ref(): Option<String
opt.as_deref(): Option<str
```
如果我们要使用as_ref()，我们还需要确保为unwrap_or()函数提供String类型，这是可行的，但不会像简单地使用as_deref()那样简洁。

