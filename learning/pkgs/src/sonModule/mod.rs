/*
todo: 传统风格的模块系统(文件夹)
src/
├── main.rs
└── my_module/
    ├── mod.rs
    └── sub_module.rs
说明：

main.rs：程序的入口文件。
my_module/：表示一个模块的目录。
mod.rs：my_module 模块的根文件。
sub_module.rs：my_module 的子模块文件。
*/

// 直接申明模块
pub mod sub_module;