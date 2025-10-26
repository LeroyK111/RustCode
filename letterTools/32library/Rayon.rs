// Rayon库使得在Rust中更容易使用并行性，它适用于将顺序迭代器转换为并行迭代器，并保证没有数据竞争。并行迭代器在运行时调整其行为以获得最大性能。

use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input
         .par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
// 在上面的示例中，只需将iter()更改为par_iter()，就可以将顺序迭代器转换为并行迭代器。