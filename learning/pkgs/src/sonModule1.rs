/*
现代风格（基于同名文件）
在 Rust 2018 版本及之后，推荐使用这种风格。模块可以直接对应同名的文件或目录，避免使用 mod.rs 文件。
示例：

src/
├── main.rs
├── my_module.rs
└── my_module/
    └── sub_module.rs
说明：

main.rs：程序的入口文件。
my_module.rs：定义 my_module 模块。
my_module/：包含 my_module 的子模块。
sub_module.rs：my_module 的子模块文件。
*/

pub mod son;