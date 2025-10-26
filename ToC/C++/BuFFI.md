# BuFFI - 简化Rust和C++之间的互操作性

这篇文章介绍了一个名为BuFFI的新工具,用于为Rust代码生成人性化和安全的C++API。BuFFI将所有类型替换为字节缓冲区,从而简化了Rust和C++两端的外部函数。作者在工作中使用该工具删除了数百行样板代码,并消除了许多可能的内存泄漏或错误指针处理。文章提供了BuFFI的链接、幻灯片和一个基本示例。该工具目前支持Rust 1.82.0工具链,未来每次Rust发布新版本时,BuFFI也会发布新版本以保持兼容性。作者希望能与社区一起解决任何潜在的兼容性问题。总的来说,BuFFI旨在简化Rust和C++之间的互操作性,提高安全性和工作效率。

https://old.reddit.com/r/rust/comments/1gouxoc/buffi_generate_ergonomic_c_apis_for_your_rust_code/