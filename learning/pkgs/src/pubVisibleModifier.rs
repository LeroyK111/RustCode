/*
todo: 特殊pub字符,

关键字	作用
pub	允许任何模块访问
pub(crate)	仅限当前 crate 内可见
pub(super)	仅限父模块可见
pub(in path)	仅限指定路径可见

*/

// !如果没有mod, 那外界引用必须使用use
pub mod pubVisibleModifier {}

pub(crate) fn internal_function() {
    /*
    什么是 crate？

    !1.Crate 是 Rust 编译单元，可以是 可执行文件（binary） 或 库（library）。

    !2.一个 Cargo.toml 代表一个 crate，整个 src/ 目录属于同一个 crate。
    */
    println!("Visible within the crate");
}

pub fn public_function() {
    internal_function();
    println!("Visible from outside the crate");
}

pub fn test() {
    println!("完全导出");
}

/*
!pub(crate) 主要用在lib中, crate中屏蔽lib中的私有函数

my_library/
├── Cargo.toml  # 定义一个库 crate
└── src/
    ├── lib.rs  # 库的主入口
    ├── internal.rs  # 内部模块
    ├── public.rs  # 公共 API

*src/lib.rs
mod internal;  // 内部模块
pub mod public; // 公共 API

*src/internal.rs
pub(crate) fn hidden_function() {
    println!("This is an internal function.");
}

*src/public.rs
use crate::internal;
pub fn public_function() {
    println!("This is a public function.");
    internal::hidden_function(); // ✅ 在 crate 内部可以调用
}

todo: 在 crate 内部可以这样调用：
fn main() {
    my_library::public::public_function(); // ✅ 访问公开 API
}

?在外部 crate（依赖 my_library 的项目）不能调用：
use my_library::internal; // ❌ 编译错误！internal 不是公有的
*/

pub(super) fn parent_only_function() {
    /*
    todo: super不结合嵌套mod, 则毫无意义
    */
    println!("Only parent module can see me!");
}

pub fn super_test() {
    parent_only_function(); // ✅ 父模块可以访问
}

pub mod parent {
    pub mod child {
        pub(super) fn parent_only_function() {
            /*
            pub(super) 使项 只能被父模块访问，但不能被更外层的模块或 crate 访问。
            */
            println!("Only parent module can see me!");
        }
    }

    pub fn test() {
        child::parent_only_function(); // ✅ 父模块可以访问
    }
}

// pub(in path)（限定路径访问）
pub(crate) fn in_path() {
    /*
    todo: super不结合嵌套mod, 则毫无意义
    */
    println!("pub(in path)（限定路径访问）");
}

pub fn in_path_test() {
    in_path(); // ✅ 父模块可以访问
}

// // todo 真实使用, 可以用来确保模块的私有性
// pub mod a {
//     pub mod b {
//         pub(in crate::a) fn restricted_function() {
//             println!("pub(in path)（限定路径访问");
//         }
//     }

//     pub fn test() {
//         b::restricted_function(); // ✅ `a` 模块内部可访问
//     }
// }

// pub fn in_path_test1() {
//     // a::test();
//     a::b::restricted_function();
// }