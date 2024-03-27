# cargo-fuzz

我们将讨论的第一个工具是cargo-fuzz，它使用一种称为模糊测试的技术来进行自动化软件测试。通过向程序提供许多有效的、几乎有效的或无效的输入，模糊测试可以帮助开发人员找到不希望看到的行为或漏洞。

当我们编写测试时，我们通常只考虑一些正常输入，并根据我们对系统反应的想象来编写测试。这种方法可能会导致遗漏错误，特别是那些由意外的或不正确的输入引起的错误。

模糊测试可以通过为程序提供各种各样的输入(包括无效的和意外的输入)来帮助你找到这些遗漏的错误。如果程序在响应这些输入时崩溃或行为异常，则表示存在错误。

cargo-fuzz crate可以对Rust代码进行模糊测试，它的工作原理是生成随机输入，并将它们输入到要测试的函数中。如果函数出现故障或崩溃，cargo-fuzz将保存导致故障的输入。

通过以下命令安装cargo-fuzz：
cargo install cargo-fuzz
下面是一个如何使用cargo-fuzz对Rust函数进行模糊测试的例子：
```toml
#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
fuzz_target!(|data: &[u8]| {

    let json_string = std::str::from_utf8(data).unwrap();
    let _ = serde_json::from_str::<serde_json::Value>(&json_string).unwrap();

});
```

上面的代码通过向JSON解析器提供随机输入来测试它。fuzz_target将持续被调用，直到遇到触发panic并导致崩溃的输入。

注意：通过模糊测试发现的一些错误可能在现实生活中不实用或不适用，这意味着模糊测试可能会产生误报。此外，模糊测试可能是资源密集型的，特别是在对大型或复杂的代码库进行模糊测试时。


# Kani

Kani是一个现代的自动代码验证工具，可以帮助你在几秒钟内验证Rust代码的正确性。它使用一种称为模型检查的技术，一种探索程序所有状态的方法，包括通过正常执行无法到达的状态。

模型检查允许Kani检测代码中的问题，这些问题可能是由意外的逻辑引起的。还可以使用Kani来识别单元测试、集成测试甚至手工测试很难或不可能发现的问题。

通过以下命令安装Kani：

通过以下命令安装Kani：
```sh
cargo install --locked kani-verifier
cargo kani setup
```
让我们看一下下面的代码：
```rs
fn product(a: i32, b: i32) -> i32 {
    a * b
}
```
上面的代码是有效的Rust代码，对吗？花点时间再看一遍——你能发现这段代码有什么可能出错的地方吗？

让我们用Kani来找出答案：
```rs
fn product(a: i32, b: i32) -> i32 {
    a * b
}


#[kani::proof]
fn main() {
    let a = kani::any();
    let b = kani::any();
    let result = product(a, b);
    println!("The product of {} and {} is {}", a, b, result);
}
```

Kani在乘法过程中发现了溢出的可能性。

这是因为product函数不能确保我们不超过i32的最大值，即2,147,483,647，任何大于该数的值都会抛出错误。本质上，无论这个函数用于什么，它都不能处理大于20亿的数字。

在这种情况下，使用Kani来识别这个潜在的问题允许您要么立即更改数据类型，要么保持原样，如果错误是预期的行为，则适当地处理错误。


# Proptest

Proptest使用大量有效和无效的输入来测试函数的属性，以发现bug。这与单元测试等经典测试方法不同，在单元测试中，指定一些输入并根据期望的行为添加断言。

属性测试是模糊测试的一种形式，它更容易控制，更侧重于验证特定的属性。这使得它成为测试复杂系统的一个很好的选择，在这些系统中，传统的模糊测试可能太慢或无效。

让我们来看看如何使用Proptest crate：
```rs
use proptest::prelude::{any, proptest};

fn add_two_numbers(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

proptest! {
    #[test]
    fn test_add_two_numbers(first_number in any::<i32>(), second_number in any::<i32>()) {
        let expected = first_number + second_number;
        let actual = add_two_numbers(first_number, second_number);
        assert_eq!(actual, expected);
    }
}
```

在上面的代码中，我们正在测试一个简单的函数，它将两个数字相加。这样一个简单的函数可能会出什么问题呢？

让我们看一下test_add_two_numbers函数签名：

```rs
fn test_add_two_numbers(first_number in any::<i32>(), second_number in any::<i32>())
```

是一个Protest中的类型，它生成随机的i32值，包括有效的和无效的。这允许我们使用广泛的输入来测试add_two_numbers()函数，包括边缘情况和异常情况。

Proptest测试函数将为first_number和second_number参数生成大量随机输入。如果任何测试失败，Proptest将把失败的输入打印到控制台。

报告显示有溢出的可能，它还显示了最小的可重复输入。有了这些信息，我们就可以继续修复bug了。

虽然属性测试可以很好地用于选定的输入范围，但它有时会遗漏一些边缘情况，并给你一个假结果。换句话说，它可能会在实际上没有错误的情况下产生错误，或者在指定的覆盖范围之外找不到错误。


# Rust KLEE

KLEE是一个符号执行引擎，它智能地探索程序中的所有代码路径，以发现漏洞或错误。它建立在LLVM编译器基础设施之上，该基础设施是用C和C++编写的。

因此，大多数KLEE实现也是用C和C++语言实现的。然而，KLEE的基本概念可以在任何编程语言中实现。

Rust Klee是Klee的开源Rust实现，被设计用来检查特定的属性。

安全检查


不变量


参数化的检查


检查Rust程序的功能正确性


Rust Klee还没有准备好用于生产，但它仍然值得一提，它是一个很酷的工具，可以帮助在Rust生态系统中形成正式的验证环境。


# Haybale

Haybale也是一个符号执行引擎，具有与Rust Klee相似的功能，Haybale完全是用Rust编写的，并且在底层基于Rust LLVM IR。

作为一个符号执行引擎，它专注于将整个程序变量转换为数学表达式，并对执行路径进行推理，以检测错误或漏洞。Haybale最好的部分是它可以测试你的Rust代码，而不需要添加额外的测试代码。

让我们看一个检查函数foo是否返回0的例子。首先，我们写出要分析的函数，你可以用任何编程语言写这个，然后把它转换成字节码：

```rs
fn foo(x: f64) -> f64 {
  x * x - 4.0
}
```


字节码将保存在项目的某个地方，你可以在Rust代码的项目变量中引用它：
```rs
let project = Project::from_bc_path("/path/to/file.bc").unwrap();
```


现在，我们可以使用haybale中的find_zero_of_func方法来发现当函数接收到零输入时存在的错误。
```rs
use haybale::{find_zero_of_func, Project};

fn main() {
  let project = Project::from_bc_path("/path/to/file.bc").unwrap();
  match find_zero_of_func("foo", &project, haybale::Config::default(), None) {
    Ok(None) => println!("foo() can never return 0"),
    Ok(Some(inputs)) => println!("Inputs for which foo() returns 0: {:?}", inputs),
    Err(e) => panic!("{}", e),
  }
}
```

Haybale可以对整个代码进行推理，发现bug，并返回一份报告，证明代码是否存在bug。虽然Haybale可能不会捕获所有错误，但它很可能会捕获导致运行时崩溃的严重错误，并给你一个修复它们的机会。
