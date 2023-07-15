# Rust学习笔记

**官网**：https://www.rust-lang.org/zh-CN/

**Rustup编译器**：https://github.com/rust-lang/rustup.rs/blob/master/README.md

**cargo三方库**：https://crates.io/  也是项目构建工具和包管理器。

**build构建库工具**：https://doc.rust-lang.org/cargo/index.html

**devTools**：vscode吧，常用一些。

**vscode插件**：https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

### windows安装方式
[安装页面](https://www.rust-lang.org/zh-CN/tools/install)
前置环境：[Microsoft C++构建工具]()
[点我下载安装包rustup.init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
配置环境变量：
![](readme.assets/Pasted%20image%2020230715203216.png)
配置成功后可以直接查看rust版本，来验证是否安装成功。
```
$ rustc --version
```

### Linux\\MacOs安装方式
直接输入，即可下载并执行安装脚本。
```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
配置环境变量
![](readme.assets/Pasted%20image%2020230715204534.png)
通过查看rust版本验证是否安装成功。

## 基本指令
### 检查rust版本
```shell
$ rustc --version
```
### 卸载rust
```shell
$ rustup self uninstall
```
### 升级rust
版本正在快速迭代，需要
```shell
$ rustup update
```
遇到的错误之一：
```
info: cleaning up downloads & tmp directories  
thread ‘main’ panicked at ‘Unable to clean up C:\Users\GrapeX.rustup\tmp: Os { code: 5, kind: PermissionDenied, message: “拒绝访问。” }’, src\utils\utils.rs:650:13  
stack backtrace:  
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
tmp无法清除，目前没有正在运行的rust项目，应当是有其它需要rust的进程还在活动，经检查 vscode 没关，rust-analyzer 还在活动，停止插件或关闭vscode即可解决。

### 高级指令
边用边记。

### Cargo
https://doc.rust-lang.org/cargo/index.html
- cargo build 可以构建项目
- cargo run 可以运行项目
- cargo test 可以测试项目
- cargo doc 可以为项目构建文档
- cargo publish 可以将库发布到 crates.io。
- cargo --version

### VScode编辑器配置
https://code.visualstudio.com/docs/languages/rust

## 入门

### Hello Rust！
### 创建新项目
```shell
cargo new learning
```
![](readme.assets/Pasted%20image%2020230715211915.png)
### 构建二进制文件
不同的系统，构建的可执行二进制文件不同。
```shell
cargo build
```
![](readme.assets/Pasted%20image%2020230715213137.png)
目录解析

### 执行主函数
![](readme.assets/Pasted%20image%2020230715213623.png)
有意思的地方来了。
当你执行cargo run时，会默认执行cargo build，然后执行exe脚本。
```shell
cargo run
```
![](readme.assets/Pasted%20image%2020230715213927.png)
### 添加三方依赖

![](readme.assets/Pasted%20image%2020230715214130.png)
坑！
```
依赖库的版本问题真的会导致很多异常和错误。
```
再次run，会自动安装依赖。
![](readme.assets/Pasted%20image%2020230715215627.png)
```rust
use ferris_says::say;

use std::io::{stdout, BufWriter};

// 引入包，并且导入say函数

  

fn test_says() {

    let stdout: std::io::Stdout = stdout();

    let message: &str = "Hello fellow Rustaceans!";

    let width: usize = message.chars().count();

  

    let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());

    say(message, width, &mut writer).unwrap();

}

  

fn main() {

    println!("Hello, Rust!");

    test_says()

}
```

然后我们吉祥物就出来了。
![](readme.assets/Pasted%20image%2020230715215843.png)
### vscode的简单配置
#### setting.json
```json
{

  // 自动检查文件编码

  "editor.detectIndentation": true,

  // 设置缩进

  "editor.insertSpaces": true,

  "editor.tabSize": 4,

  // 文件默认编码格式

  "files.encoding": "utf8",

  // 自动保存

  "files.autoSave": "onFocusChange",

  // 保存时格式化

  "editor.formatOnSave": true,

  // 粘贴时格式化

  "editor.formatOnPaste": true,

  // 边写边格式

  "editor.formatOnType": true,

  "editor.fontFamily": "Fira Code", //后边的引号中写上要设置的字体类型，个人比较喜欢Fira Code

  "editor.fontLigatures": true, //这个控制是否启用字体连字，true启用，false不启用，这里选择启用

  "editor.fontSize": 14, //设置字体大小，这个不多说都明白

  "editor.fontWeight": "normal", //这个设置字体粗细，可选normal,bold,"100"~"900"等，选择合适的就行

  "editor.wordWrapColumn": 255,

  "files.autoGuessEncoding": true,

  // 开启自动换行

  "editor.wordWrap": "on",

  // 更改行号颜色

  "workbench.colorCustomizations": {

    "editorLineNumber.foreground": "#daa520"

  },

  // 开启rust插件，代码灰块提示。

  "editor.inlayHints.enabled": "on",

  // todo 语意语法提示块，高亮

  "editor.semanticTokenColorCustomizations": {

    "rules": {

      "*.mutable": {

        "fontStyle": ""

      },

    }

  },

}
```
#### 调试
要开始调试，您首先需要安装具有调试支持的两种语言扩展之一：

- [Microsoft C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) (ms-vscode.cpptools) –_在 Windows 上_
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (vadimcn.vscode-lldb) –_在 macOS/Linux 上_

如果您忘记安装这些扩展之一，当您尝试启动调试会话时，rust-analyzer 将提供一条通知，其中包含 VS Code Marketplace 的链接。

设置断点, 选择调试器。
![](readme.assets/Pasted%20image%2020230715222428.png)
![](readme.assets/Pasted%20image%2020230715222636.png)