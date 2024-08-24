# Rustls 

意味着现在Rustls可以无缝替换目前世界上使用的最广泛的Web Server - Nginx 中了。

# Servo浏览器引擎事件循环的修复和优化

浏览器


# 在nodejs中使用rust

要在Node.js中使用Rust，通常需要在Rust中创建一个本地Node.js模块。有几种方法可以实现这种集成，包括使用像neon、napi-rs、FFI和WebAssembly (WASM)这样的库。

每种方法都有其优点，选择取决于您的应用程序的具体需求：

- **Neon：**最适合与Node.js直接集成，提供了一种简单有效的方式来编写本机模块。
    
- **NAPI-RS：**适用于使用Node-API创建稳定的、与版本无关的本地模块。
    
- **WebAssembly****：**非常适合在Node.js和浏览器中运行Rust代码，提供可移植性和性能。
    
- **FFI：**对于需要直接从Node.js调用Rust函数而不需要额外绑定的场景非常有用。

## 使用Neon

Neon是一个库，为在Rust中编写本地Node.js模块提供绑定。它简化了Rust与Node.js集成的过程，可以在JavaScript应用程序中利用Rust的性能和安全优势。

示例：用Neon创建一个简单的Rust模块

安装Neon CLI

```
npm install -g neon-cli
```

创建一个新的Neon项目

```
neon new my-neon-projectcd my-neon-project
```

编写Rust代码

在src/lib.rs文件中添加一个简单的函数：

```
use neon::prelude::*;fn hello(mut cx: FunctionContext) -> JsResult<JsString> {    Ok(cx.string("Hello from Rust!"))}register_module!(mut cx, {    cx.export_function("hello", hello)});
```

构建项目

```
neon build
```

在Node.js中使用Module

```
const addon = require('../native');console.log(addon.hello()); // 输出: Hello from Rust!
```


## 使用NAPI-RS

NAPI-RS是另一个用Rust编写Node.js原生插件的流行库。它使用Node-API (N-API)，它为Node.js模块提供了一个稳定的ABI(应用程序二进制接口)。这确保了不同版本Node.js的兼容性。

示例：使用NAPI-RS创建一个简单的Rust模块

安装NAPI-RS CLI

```
npm install -g @napi-rs/cli
```

创建一个新的NAPI-RS项目

```
napi new my-napi-projectcd my-napi-project
```

编写Rust代码

在src/lib.rs文件中添加一个简单的函数：

```
#[macro_use]extern crate napi_derive;#[napi]fn hello() -> String {    "Hello from Rust!".to_string()}
```

构建项目

```
napi build
```

在Node.js中使用Module

```
const { hello } = require('./napi-rs');console.log(hello()); // 输出: Hello from Rust!
```

## 使用WebAssembly(WASM)

WASM是在Node.js应用程序中使用Rust的另一种方法。WASM允许你将Rust代码编译成可以在Node.js运行时执行的二进制格式。

示例：创建简单WASM模块

安装wasm-pack

```
cargo install wasm-pack
```

创建一个新的项目

```
cargo new --lib wasm_examplecd wasm_example
```

添加WASM target

在Cargo.toml文件中加入以下内容：

```
[lib]crate-type = ["cdylib"][dependencies]wasm-bindgen = "0.2"
```

编写Rust代码

在src/lib.rs中添加Rust函数：

```
use wasm_bindgen::prelude::*;#[wasm_bindgen]pub fn greet(name: &str) -> String {    format!("Hello, {}!", name)}
```

构建项目

```
wasm-pack build --target nodejs
```

在Node.js中使用Module

```
const { greet } = require('./pkg/wasm_example');console.log(greet('World'));
```


## 使用FFI

另一种方法是使用FFI从Node.js调用Rust函数。这种情况不太常见，但对于首选直接绑定的某些场景可能很有用。

示例：创建一个简单的Rust库

创建一个Rust库

```
cargo new --lib my_rust_librarycd my_rust_library
```

添加构建目标

在Cargo.toml文件中加入以下内容

```
[lib]crate-type = ["dylib"]
```

编写Rust代码

在src/lib.rs中添加Rust函数

```
#[no_mangle]pub extern "C" fn hello() -> *const u8 {    "Hello from Rust!".as_ptr()}
```

编译库

```
cargo build --release
```

在Node.js中使用库

```
const ffi = require('ffi-napi');const path = require('path');const lib = ffi.Library(path.join(__dirname, 'target/release/libffi'), {    'hello': ['string', []]});console.log(lib.hello()); // 输出: Hello from Rust!
```