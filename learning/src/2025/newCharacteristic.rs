
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