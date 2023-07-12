# Rust学习笔记

**官网**：https://www.rust-lang.org/zh-CN/

**Rustup编译器**：https://github.com/rust-lang/rustup.rs/blob/master/README.md

**cargo三方库**：https://crates.io/

**build构建库工具**：https://doc.rust-lang.org/cargo/index.html

**devTools**：vscode吧，常用一些。

**vscode插件**：https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

## 入门

目录模板：

```
hello-rust # cargo new hello-rust
|- Cargo.toml # Cargo.toml为 Rust 的清单文件。其中包含了项目的元数据和依赖库。
|- src # 核心代码
  |- main.rs
```

```
# 执行代码
$ cargo run
```

添加依赖项目，并重新构建。

```
# Cargo.toml修改依赖，跟js的packages.json一样
[dependencies]
ferris-says = "0.2"
```

```
# 构建
$ cargo build
```

导出包中的函数，main.rs

```
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
```

重启

```
$ cargo run
```

## 基础语法















