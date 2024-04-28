# 一个声明式解析库 Untwine 发布0.4
Untwine是一个声明性解析库，它允许一种类似于使用自定义宏语法进行直接模式匹配的解析风格。这允许创建具有良好性能特征和高质量错误消息的极其紧凑的解析器。这些解析器实现起来很简单，有几个精心挑选的例子：

一个几乎完整的JSON解析器，包含12行解析逻辑
支持除特殊转义序列之外的所有基本JSON功能（除“）
一个在6行解析逻辑中具有四运算表达式解析器的pmdas
一个辅助函数对两个数字进行运算
使用untwine制作的解析器也有高质量的错误消息，可以直观地显示错误和相关语法。

fn operate(left: f64, op: char, right: f64) -> f64 {
    match op {
        '+' => left + right,
        '-' => left - right,
        '/' => left / right,
        '*' => left * right,
        _ => unreachable!(),
    }
}

parser! {
    sep = #{char::is_ascii_whitespace}*;
    num: num=<"-"? '0'-'9'+ ("." '0'-'9'+)?> -> f64 { num.parse().unwrap() }
    term = (num | "(" sep expr sep ")") -> f64;
    add: first=mul sep ops=(["+-"] sep mul)* -> f64 { ops.into_iter().fold(first, |left, (op, right)| operate(left, op, right)) }
    mul: first=term sep ops=(["*/"] sep term)* -> f64 { ops.into_iter().fold(first, |left, (op, right)| operate(left, op, right)) }
    pub expr = add -> f64;
}
更多信息查看 GitHub，https://github.com/boxbeam/untwine

# 用Rust从头实现一个C编译器
一个针对 MacOs 和 Linux 的 x86-64 的 C99 编译器，它没有任何依赖项，并且是独立的，因此可以通过单个命令进行安装（请参阅安装）。

更多信息查看 GitHub，https://github.com/PhilippRados/wrecc


# Exhaustive 实现一个类型的所有值的测试
这个crate提供了一个特性来生成一个类型的所有值（达到一定深度）。它还提供了一个派生宏，将为您派生此特性。最后，它提供了一个测试宏，用于对某个类型的所有值运行基于属性的测试。

#[derive(Debug, Exhaustive)]
enum Test1 { A, B, }
#[derive(Debug, Exhaustive)]
struct Test2 { a: bool }

#[exhaustive_test]
fn test(v: Test1, w: Test2) {
    println!("{v:?} {w:?}");
}


# Rust优于C++的两个原因

## 原因1：积极的编译器优化

- Rust的编译器(LLVM)比C++编译器更积极地优化代码，这是因为所有权规则，LLVM可以做出假设。
- LLVM在内联函数方面更加积极，特别是对于小函数。内联避免了函数调用开销并使其快速。

例如，下面的函数可能会或可能不会被C++编译器内联，但LLVM肯定会内联它。

```rust
fn f(n: i32, dp: &mut Vec<i32>) -> i32 {    let n1 = n as usize;    if dp[n1] != -1 {        return dp[n1];    }    dp[n1] = Self::f(n-1, dp) + Self::f(n-2, dp) + Self::f(n-3, dp);    dp[n1]}
```


## 原因2：较低的运行时开销

### 1，C++栈展开导致运行缓慢

什么是栈展开？

每当抛出异常时，在栈上开始分配资源和调用对象的析构函数的过程，这称为栈展开。

```c++
class Resource {public:    Resource() {        std::cout << "Resource acquired\n";    }    ~Resource() {        std::cout << "Resource released\n";    }};void foo() {    Resource res; // Resource acquired    throw std::runtime_error("Error in foo");}int main() {    try {        foo();    } catch (const std::runtime_error& e) {        std::cerr << "Caught exception: " << e.what() << std::endl;    }    return 0;}
```

栈展开是如何工作的？

- 调用foo()时，它获取一个Resource对象。
    
- 然后抛出std::runtime_error异常。
    
- 作为结果，栈开始展开，并调用res的析构函数来释放Resource。
    
- 然后，在main()函数中捕获异常。
    
- 这确保了即使在出现异常的情况下也能正确地清理资源。
    
栈展开有运行时开销，当存在深度嵌套的函数调用或具有复杂析构函数的对象时，将花费时间来释放对象。

### 2，Rust使用Result和Option类型删除了的栈展开

Rust的Result和Option类型用于错误处理，通过模式匹配而不是异常来处理。

```rust
fn divide(a: i32, b: i32) -> Result {    if b == 0 {        return Err("Division by zero");    }    Ok(a / b)}fn main() {    match divide(10, 0) {        Ok(result) => println!("Result: {}", result),        Err(e) => eprintln!("Error: {}", e),    }}
```

### 3，C++运行时类型信息(RTTI)增加了二进制大小和运行时开销

增加二进制大小：

运行时类型信息(RTTI)，RTTI意味着在运行时执行动态类型检查和类型转换。当启用RTTI时，编译器在二进制文件中包含额外的元数据以支持动态类型信息。

这些元数据通常包括：类型信息表(类型描述符)、用于动态调度等的虚函数表(vtable)。这些表增加了二进制文件的大小，特别是对于具有大量多态类的程序。

增加执行时间：

动态强制转换(dynamic_cast)，这包括运行时类型检查，以确保转换的正确性。这种类型检查增加了程序执行时间的开销。

虚函数调用，C++语言中的动态多态性适用于虚函数调用，这需要在运行时查找适当的函数。与静态调度相比，会产生额外的运行时开销。

```rust
#include <iostream>#include <typeinfo>class Base {public:    virtual ~Base() {}};class Derived : public Base {};int main() {    Base* ptr = new Derived();    Derived* derived = dynamic_cast(ptr);    if (derived) {        std::cout << "Dynamic cast successful\n";    } else {        std::cout << "Dynamic cast failed\n";    }    delete ptr;    return 0;}
```

### 4，Rust中没有RTTI ???

Rust的类型系统支持多态行为和动态分派(基于trait和enum)，而不需要RTTI。Box启用动态分派，不需要运行时类型信息。

```rust
trait Printable {    fn print(&self);}struct Base;struct Derived;impl Printable for Base {    fn print(&self) {        println!("Base");    }}impl Printable for Derived {    fn print(&self) {        println!("Derived");    }}fn main() {    let base: Box = Box::new(Derived);    base.print();}
```

Rust的编译器建立在LLVM上，将高级结构转换为高效的机器码。