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