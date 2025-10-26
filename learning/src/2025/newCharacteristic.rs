
// 1，包括新的常量功能，比如内联常量表达式，它可以直接在表达式中定义和使用常量，而不需要单独声明；
// let foo = [const {None}; 100]

// 2，我们还可以使用可变引用来引用静态变量；
// static S: i32 = 25;
// const C: &32 = &S;


// 3，在const上下文中使用原始指针和内部可变性；
// const fn inc(x: &mut i32) {
//     *x += 1;
// }



// use std::cell::Unsafecell;
// const C: i32 = {
//     let c = Unsafecell::new(41);
//     unsafe { *c.get() += 1 };
//     c.into_inner()
// }



// 4，LazyCell和LazyLock
// use std::sync::LazyLock;
// use std::time::Instant;
// static LAZY TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

// fn main() {
//     let start = Instant::now();
//     std::thread::scope(|s| {
//         s.spawn(l {
//         println!("Thread lazy time is {:?}", LAZY_TIME.duration_since(start))
//         });
//         println!("Main lazy time is {:?}", LAZY TIME.duration since(start))
//     });
// }



// 但最令人兴奋的更新是Rust 2024 Edition，它将包括保留Gen关键字，以便将来可以添加生成器。生成器提供了一种使用命令式语法创建迭代器的方法，或者换句话说，描述一系列动作，最大的好处是它们简化了复杂迭代器的创建。
// fn primes() -> impl Iterator<Item = usize> {

//     gen {
//         let mut sieve = vec![true; 10000];
//         for num in 2..sieve.len() {
//             if sieve[num] {
//                 yield num;
//                 for multiple in (num * 2)..sieve.len() {
//                     sieve[multiple] = false;
//                 }
//             }
//         }
//     }

// }



/*
新增 If-Let Chain 功能，允许在 if 和 while 条件中使用 && 连接 let 语句，简化代码结构，但仅在Rust 2024版中可用。if let Channel::Stable(v) = release_info()
    && let Semver { major, minor, .. } = v
    && major == 1
    && minor == 88
{
    println!("`let_chains` was stabilized in this version");
}
支持 naked functions，通过 #[unsafe(naked)] 属性和 naked_asm! 宏，实现对函数汇编代码的完全控制，适用于系统软件开发。#[unsafe(naked)]
pub unsafe extern "sysv64" fn wrapping_add(a: u64, b: u64) -> u64 {
    // Equivalent to `a.wrapping_add(b)`.
    core::arch::naked_asm!(
        "lea rax, [rdi + rsi]",
        "ret"
    );
}
cfg 配置语言现支持布尔值 true 和 false，使条件编译更直观。Cargo引 入自动缓存清理机制，自动清理长时间未访问的缓存文件，减少磁盘占用，但离线模式除外。此外，还稳定了多个 API，包括 Cell::update、HashMap::extract_if 等，并在常量上下文中稳定了一些之前稳定的 API。
*/