# 从其他Rust代码生成Rust代码，而不是Rust编译器的代码生成步骤。源代码生成的另一个术语是元编程，但这里将其称为动态代码生成。

它能解决什么问题？

桌面应用程序通常把一个嵌入到Rust二进制文件中的web前端交付给终端用户。像Tauri这样的项目通过编写Rust代码，生成了更多的Rust代码，来实现嵌入代码的生成。

假设web前端的输出是这样的：
```shell
dist
├── assets
│  ├── script-44b5bae5.js
│  ├── style-48a8825f.css
├── index.html
```
让我们使用include_str!宏将这些文件嵌入到Rust项目中，它将指定的文件内容添加到二进制文件中。它看起来像这样：

```rust
use std::collections::HashMap;

fn main() {
    let mut assets = HashMap::new();

    assets.insert(
        "/index.html",
        include_str!("../dist/index.html")
    );

    assets.insert(
        "/assets/script-44b5bae5.js",
        include_str!("../dist/assets/script-44b5bae5.js")
    );

    assets.insert(
        "/assets/style-48a8825f.css",
        include_str!("../dist/assets/style-48a8825f.css")
    );
}
```
非常简单，现在我们可以直接从最终二进制文件中获取这些资源了！然而，如果我们并不总是提前知道资源的文件名，该怎么办呢？假设我们在前端项目上做了更多的工作，现在它的输出是这样的：

```sh

dist
├── assets
│  │ # script-44b5bae5.js previously
│  ├── script-581f5c69.js
│  │
│  │ # style-48a8825f.css previously
│  ├── style-e49f12aa.css
├── index.html
```

我们资源的文件名已经改变了，因为我们的前端打包器使用了缓存破坏机制。在我们修复其中的文件名之前，Rust代码不再编译。

如果我们每次更改前端都必须更新Rust代码，这将是一种糟糕的开发体验——想象一下，如果我们有几十个资源！

Tauri使用动态代码生成来避免这种情况，它在编译时查找资源，并生成调用正确资源的Rust代码。

## 工具

让我们讨论一些用于代码生成的工具，然后使用它们来实现我们自己的资源捆绑器。

quote crate：使我们能够编写转换成数据的Rust代码，然后生成语法正确的Rust代码。这个crate在Rust生态系统中无处不在，用于代码生成。

walkdir crate：提供了一种简单的方法来递归地抓取目录中的所有项

phf crate：使用完美的散列函数实现HashMap

Rust代码生成通常发生在构建脚本或宏中，我们将使用构建脚本构建简单的资源捆绑器，因为我们将访问磁盘。

### 构建资源捆绑器

创建Rust库

让我们从创建一个新的Rust库开始：
cargo new --lib asset-bundler
我们希望为使用该库的应用程序创建一种获取资源的方法，所以让我们首先创建它。

加入phf依赖项：
cargo add phf --features macros
在src/lib.rs文件中，写入以下代码：
```rust
pub use phf; // 重新导出phf，以便我们以后使用
type Map = phf::Map<&'static str, &'static str>;

/// 用于编译时嵌入资源的容器
pub struct Assets(Map);

impl From<Map> for Assets {
    fn from(value: Map) -> Self {
        Self(value)
    }
}

impl Assets {
    /// 获取指定资源
    pub fn get(&self, path: &str) -> Option<&str> {
        self.0.get(path).copied()
    }
}
```
对于Assets结构体，我们不需要太多的功能。在这里我们创建了一个关于phf::Map的封装器和一个让调用者获得内容的方法。

代码生成器

现在，我们创建一个在构建脚本中使用的库，以生成代码。

因为我们将在同一个项目中拥有多个crate，因此需要将项目转换为cargo workspace。修改Cargo.toml文件，如下：
```toml
[workspace]
members = ["codegen"]

[package]
name = "asset-bundler"
version = "0.1.0"
edition = "2021"

[dependencies]
phf = { version = "0.11.2", features = ["macros"] }
```
在项目根目录下，运行以下命令来创建codegen项目并加入依赖项：
```sh
cargo new --lib codegen --name asset-bundler-codegen
cargo add quote walkdir --package asset-bundler-codegen
```
在codegen项目中，我们需要完成的功能如下：

将一个资源路径传递给我们的函数，我们将其称为base。

检查base是否存在


递归地收集base中的所有文件路径。

生成嵌入所有文件路径的代码。

最后要提到的是，我们希望通过传递一个相对路径来获取资源。我们想要的是assets.get("index.html")，而不是assets.get(". /dist/index.html")，这意味着我们需要跟踪传入函数的base目录。

让我们把这些需求写在codegen/src/lib.rs的代码中：
```rust
use std::path::{Path, PathBuf};

/// 生成Rust代码
pub fn codegen(path: &Path) -> std::io::Result<String> {
    // canonicalize 会检查路径是否存在
    let base = path.canonicalize()?;

    let paths = gather_asset_paths(&base);
    Ok(generate_code(&paths, &base))
}

/// 递归地查找传递目录中的所有文件
fn gather_asset_paths(base: &Path) -> Vec<PathBuf> {
    todo!()
}

/// 生成代码
fn generate_code(paths: &[PathBuf], base: &Path) -> String {
    todo!()
}
```
让我们先来看一下gather_assets_path函数，我们将使用walkdir从传入的base目录中递归地抓取所有文件。这里使用了flatten()，它删除了嵌套迭代器。代码实现如下：
```rust
/// 递归地查找传递目录中的所有文件
fn gather_asset_paths(base: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(base).into_iter().flatten() {
        // 我们只关心文件，忽略目录
        if entry.file_type().is_file() {
            paths.push(entry.into_path())
        }
    }

    paths
}
```
现在我们有了一个应该在二进制文件中包含的所有资源文件的列表。

接下来，我们需要从所有路径中去掉我们之前解析的base前缀，代码如下：
```rust
/// 将路径转换为适合的相对路径
fn keys(paths: &[PathBuf], base: &Path) -> Vec<String> {
    let mut keys = Vec::new();

    for path in paths {
        if let Ok(key) = path.strip_prefix(base) {
            keys.push(key.to_string_lossy().into()
        }
    }

    keys
}
下面实现代码生成函数generate_code：
/// 生成代码
fn generate_code(paths: &[PathBuf], base: &Path) -> String {
    let keys = keys(paths, base);
    let values = paths.iter().map(|p| p.to_string_lossy());

    // 双括号使其成为块表达式
    let output = quote! {{
        use ::asset_bundler::{Assets, phf::{self, phf_map}};
        Assets::from(phf_map! {
            #( #keys => include_str!(#values) ),*
        })
    }};

    output.to_string()
}
```
在这里，我们使用了quote库的宏，它允许我们同时使用键和值两个集合。

测试

我们刚刚制作了一个简单的资源捆绑器，现在是时候使用它了！我们将从创建一个新的example项目开始，以使用我们刚刚创建的两个库。

首先，修改根目录下的Cargo.toml文件：
```toml
[workspace]
members = ["codegen", "example"]
```
然后，我们创建example二进制文件并添加依赖项：
```sh
cargo new --bin example
cargo add asset-bundler --path . --package example
cargo add --build asset-bundler-codegen --path codegen --package example
touch example/build.rs
mkdir -p example/assets/scripts
```
让我们从构建脚本example/build.rs开始，我们需要调用前面创建的codegen函数来获取生成的代码。代码如下：
```rust
use std::path::Path;

fn main() {
    let assets = Path::new("assets");
    let codegen = match asset_bundler_codegen::codegen(assets) {
        Ok(codegen) => codegen,
        Err(err) => panic!("failed to generate asset bundler codegen: {err}"),
    };

    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out).join("assets.rs");
    std::fs::write(out, codegen.as_bytes()).unwrap();
}
```
我们最终将代码写入$OUT_DIR/assets。

我们需要创建一些资源，让我们假设这些资源是用于web服务器的，而这些文件是提供给浏览器的。运行以下命令创建它们：
```sh
echo -n "scripts/loader-a1b2c3.js" > example/assets/index.html
echo -n "scripts/dashboard-f0e9d8.js" > example/assets/scripts/loader-a1b2c3.js
echo -n "console.log('dashboard stuff')" > example/assets/scripts/dashboard-f0e9d8.js
```
然后在example/src/main.rs文件中，写入以下代码：
```rust
fn main() {
    // 包含构建脚本创建的资源
    let assets = include!(concat!(env!("OUT_DIR"), "/assets.rs"));
    println!("-------->assets = {:?}", assets);
    let index = assets.get("index.html").unwrap();
    let loader = assets.get(index).unwrap();
    let dashboard = assets.get(loader).unwrap();

    assert_eq!(dashboard, "console.log('dashboard stuff')");
}
```
总结

像Tauri框架就广泛地使用代码生成技术来执行代码注入、压缩和验证绑定的资源。动态代码生成是一个强大的工具，可以为Rust程序带来高级功能。