/*
Rust 1.77.0 版本发布
Rust 团队宣布了 Rust 编程语言的最新版本 1.77.0。这个版本虽然相对小，但包含了一系列改进，使得 Rust 语言更加强大和易用。
*/

//C-string 字面量: Rust 现在支持 C-string 字面量（例如 c"abc"），它们会扩展为内存中以 null 字节终止的字符串，类型为 &'static CStr。这对于编写与需要 null 终止字符串的外语接口互操作的代码非常方便，所有相关的错误检查（例如，内部 null 字节的缺失）都会在编译时进行。
let c_string: &CStr = c"Hello, world!";




// 支持异步函数递归: 在 1.77 版本中，异步函数现在可以递归调用自己，只要使用某种形式的间接方式来避免函数状态的无限大小。
async fn fib(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => Box::pin(fib(n - 1)).await + Box::pin(fib(n - 2)).await,
    }
}

// 稳定化的 offset_of! 宏: 1.77.0 版本稳定化了 offset_of! 宏，该宏提供了访问结构体字段字节偏移量的方法。这在使用类型实例之前需要字段偏移量时特别有用。


let offset: usize = offset_of!(StructName, field);


/*
在非调试配置中默认启用 strip: 没有启用调试信息（例如，debug = 0）的 Cargo 配置现在默认会启用 strip = "debuginfo"。
*/

/*
Clippy 新增 incompatible_msrv 检查: 为了避免开发者在编写代码时不小心使用了比声明的最小支持 Rust 版本（MSRV）更新的 API，Clippy 新增了一个 incompatible_msrv 检查。
*/

/*
稳定化的 API
array::each_ref 和 array::each_mut
core::net
f32::round_ties_even 和 f64::round_ties_even
一系列切片操作函数，如 slice::first_chunk, slice::last_chunk, slice::chunk_by 等。
Bound::map
File::create_new
Mutex::clear_poison 和 RwLock::clear_poison
*/