# 为什么不鼓励’&String, &Vec或&Box'作为Rust中的函数参数？

在Rust中，在设计函数时，通常通过引用传递参数，以避免不必要的克隆或所有权转移。但是，建议避免使用&String、&Vec或&Box等类型作为函数参数。

## 问题的根源：冗余和有限的灵活性

1. 间接冗余像String、Vec<T>或Box<T>这样的类型已经是堆分配的，并且提供了间接的访问。当使用&String、&Vec或&Box时，不必要地增加了另一层间接访问。

```rust

fn print_length(string_ref: &String) {
    println!("Length: {}", string_ref.len());
}

```
在这里，&String意味着你正在传递一个对堆分配的String的引用。但是String已经有了一个借用视图的概念-&str。更习惯的用法是：
```rust

fn print_length(string_slice: &str) {
    println!("Length: {}", string_slice.len());

}

```

为什么这样更好？我们很快就会讲到。

1. 丧失一般性&String或&Vec<T>作为参数，限制你的函数只接受对这些特定类型的引用。这限制了它接受其他类似方式的借用类型的可用性。考虑&str和&String，&String仅适用于对String的引用，但&str可以用于字符串字面值(&'static str)、子字符串或任何实现解引用成str的类型。
 ```rust
fn print_message(message: &String) {
    println!("{}", message);
}


fn main() {
    let owned_string = String::from("Hello, world!");
    print_message(&owned_string); // Works

    let literal = "Hello, world!";
    // print_message(literal); // ERROR: 类型不匹配
}
```
现在，如果我们重构函数以接受&str：
```rust
fn print_message(message: &str) {
    println!("{}", message);
}

fn main() {
    let owned_string = String::from("Hello, world!");
    print_message(&owned_string); // Works

    let literal = "Hello, world!";
    print_message(literal); // Works!
}
```
这使得该函数更加通用。

1. 强制解引用的力量Rust的deref自动将对类型(如&string或&vec <T>)的引用强制转换为对其“内部”类型(&str或&[T])的引用。这就是为什么你不需要显式地编写接受&String或&Vec的函数——编译器会为你处理转换。

```rust
fn print_items(items: &[i32]) 
{
    for item in items {
        println!("{}", item);
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    print_items(&vec); // &vec<i32>自动强制转换为&[i32]
}
```
通过接受最通用的形式(&str, &[T])作为函数参数，我们将受益于这个强大的特性。

函数参数的最佳实践1. 使用切片(&[T])代替&Vec<T>切片(&[T])表示为一个连续的元素序列视图，这正是Vec所提供的。通过接受切片，函数可以处理任何可以解引用为切片的类型，而不仅仅是Vec。

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}
```
这适用于数组、向量和其他类切片类型。

1. 使用&str代替&String&str是一个字符串切片，它可以以各种形式表示字符串数据——字符串字面值、字符串的一部分或整个字符串。通过使用&str，函数变得更加通用和灵活。
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```
这个函数可以同时处理字符串和字符串字面量。

1. 避免使用&Box<T>Box<T>是堆分配的，但是使用&Box<T>引入了冗余的间接访问。我们可以简单地用&T来代替。
 
```rust
fn process(data: &Box<i32>) {
    println!("{}", data);
}

fn process(data: &i32) {
    println!("{}", data);
}
```
什么时候应该使用&String， &Vec或&Box？虽然在大多数情况下不鼓励这样做，但在极少数情况下，可能需要接受&String，&Vec或&Box：
1，正在使用显式要求这些类型的api
2，正在调试或重构现有代码，并且不能更改上游设计。

即便如此，如果可能的话，也要考虑重构API。