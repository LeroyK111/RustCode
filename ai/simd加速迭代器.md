SIMD 加速的迭代器
单指令流多数据流（Single Instruction Multiple Data，缩写：SIMD）是一种采用一个控制器来控制多个处理器，同时对一组数据（又称"数据向量"）中的每一个分别执行相同的操作从而实现空间上的并行性的技术。

原先使用标准库中的 SIMD，较为麻烦，有了 simd-itertools 这个库，就变得非常轻松了，


`arr.iter().contains()`
修改为

`arr.iter().contains_simd()`

即可获取 simd 加速效果。

目前该库已经支持 find filter position contains eq min/max is_sorted all_equal 等方法，支持u8,u16,u32,u64,i8,i16,i32,i64,f32,f64,isize,usize 这些数据类型。

其 benchmark 与标准库中的迭代器进行了对比，数据表明，被迭代的数据向量越长，使用 simd 加速的 iterator 带来的性能提升越明显（最终趋于稳定）