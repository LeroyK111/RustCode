// cfg-if crate以if-else表达式的形式为依赖于大量#[cfg]配置的项提供了方便的声明。

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        fn foo() { /* unix specific functionality */ }
    } else if #[cfg(target_pointer_width = "32")] {
        fn foo() { /* non-unix, 32-bit functionality */ }
    } else {
        fn foo() { /* fallback implementation */ }
    }
}