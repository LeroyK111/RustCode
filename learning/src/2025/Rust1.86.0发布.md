# Rust 1.86.0 发布

Rust 团队发布了 Rust 1.86.0 版本，通过  rustup update stable  可更新。该版本带来多项重要更新，包括支持 trait 向上转型、HashMap  和切片可同时获取多个元素的可变引用、安全函数可使用  #[target_feature]  属性、新增指针非空调试断言、missing_abi lint 默认警告、1.87.0 版本将弃用  i586-pc-windows-msvc  目标平台，还稳定了一批 API。
## 新特性
    
- **trait 向上转型**：实现了 trait 对象的向上转型，若 trait 存在上级 trait，可将该 trait 对象的引用强制转换为上级 trait 对象的引用，如 `Arc<dyn Trait> -> Arc<dyn Supertrait>` 等。此前需在 `trait` 内定义 `upcast` 方法实现，且仅适用于一种引用 / 指针类型，现在不再需要。使用 `Any` trait 时，该特性可将 trait 对象向上转型为 `dyn Any`，调用 `Any` 的 `downcast` 方法 。
    
- **`HashMap`和切片支持多元素可变索引**：标准库提供 `get_disjoint_mut` 辅助函数，切片和 `HashMap` 可同时获取多个元素的可变引用，解决了借用检查器阻止重复调用 `get_mut` 方法获取的引用同时使用的问题。
    
- **安全函数使用 `#[target_feature]` 属性**：稳定了 `target_feature_11` 特性，允许安全函数使用 `#[target_feature]` 属性。标记该属性的安全函数，只能在同样标记了该属性的函数中安全调用；在未标记的函数中调用时，需在 `unsafe` 块内进行，且调用者要确保目标特性可用。
    
## 增强与变更
    

- **指针非空调试断言**：编译器会在非零大小读写和指针重新借用为引用时，插入指针非空的调试断言。如 `let _x = *std::ptr::null::<u8>();` 在启用调试断言时会触发非展开式 panic。该断言仅在启用调试断言时生效，不能依赖其保证程序正确性。
    
- **`missing_abi` lint 默认警告**：在 `extern` 块和函数中省略 ABI（如`extern {}`和`extern fn`）会触发 `missing_abi` lint 警告。建议显式指定 `"C"` ABI（如`extern "C" {}`和`extern "C" fn`）。
    

- **目标平台变更**：1.87.0 版本将移除 `tier-2` 目标平台 `i586-pc-windows-msvc`，因其与 `i686-pc-windows-msvc` 的区别在于不要求 SSE2 指令支持，但 Windows 10（除`win7` 目标外所有 `windows` 目标的最低要求操作系统版本）本身需要 SSE2 指令。使用该目标平台的用户应在 1.87.0 发布前迁移至 `i686-pc-windows-msvc`。
    
## 稳定的 API
    
- **新稳定 API**：包括 `{float}::next_down`、`{float}::next_up`、`[T]::get_disjoint_mut` 等多个 API。
    
- **const 上下文稳定的 API**：如 `hint::black_box`、`io::Cursor::get_mut` 等 API 在 const 上下文中稳定。
