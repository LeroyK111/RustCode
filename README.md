# Rust学习笔记

自举发展
![](https://raw.githubusercontent.com/LeroyK111/pictureBed/master/20250323181138.png)

**官网**：https://www.rust-lang.org/zh-CN/

**Rustup编译器**：https://github.com/rust-lang/rustup.rs/blob/master/README.md

**cargo三方库**：https://crates.io/  也是项目构建工具和包管理器。

**build构建库工具**：https://doc.rust-lang.org/cargo/index.html

**devTools**：VScode吧，常用一些。

**vscode插件**：https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer

``` markdown
# 底层语言的原理

硬件并不是天生支持C语言，而是C语言被设计为一种能够方便地映射到底层硬件的语言。C语言的设计初衷之一是提供一种能够直接操作计算机硬件的工具，使得程序员能够更容易地编写系统级和底层代码。

C语言的底层支持主要体现在以下几个方面：

1. 直接的内存访问： C语言支持指针，使得程序员能够直接操作内存地址。这使得C程序可以更灵活地进行内存管理，直接访问和操作底层硬件的内存区域。
    
2. 数据类型和操作符的底层映射： C语言的基本数据类型和操作符通常能够直接映射到底层硬件的指令集。例如，整数类型和算术操作可以直接对应到底层的整数运算指令。
    
3. 底层控制流： C语言提供了底层的控制流结构，如条件语句、循环语句等，可以直接映射到底层硬件的分支和跳转指令。
    
4. 函数调用和栈管理： C语言的函数调用和栈管理机制能够映射到硬件的函数调用和栈操作。这使得C程序可以有效地利用硬件的栈来管理函数调用和局部变量。
    

虽然C语言具有较好的底层支持，但也需要注意，不同的硬件平台和体系结构有着不同的指令集和体系结构特性。因此，编写与底层硬件交互的代码时，程序员可能需要考虑硬件的具体特点，并在一些情况下使用平台相关的特性。

# rust

Rust的设计目标之一是提供内存安全和并发安全，同时保持与底层硬件的直接交互能力。与C语言一样，Rust允许直接访问底层硬件，但它在语言级别提供了更多的安全性和抽象。Rust实现与硬件的交互主要通过以下几个方面：

1. 所有权和借用系统： Rust引入了所有权和借用系统，用于管理内存的生命周期和所有权转移。这种系统确保在编译时就能够检测到潜在的内存错误，例如空指针引用、数据竞争等。这使得Rust程序在直接访问底层硬件时更容易避免常见的安全问题。
    
2. `unsafe`块： 尽管Rust强调安全性，但它也允许使用`unsafe`块来进行一些编写不安全代码的操作，比如直接操作原始指针、进行底层的内存布局等。`unsafe`块提供了一种标记机制，通知编译器在这些块中的代码可能存在潜在的危险，需要程序员负责确保安全性。

3. 内联汇编： Rust通过`asm!`宏允许插入内联汇编代码，以便在需要的情况下直接使用汇编语言进行底层操作。

4. 裸机开发支持： Rust提供了支持裸机开发（bare-metal）的能力，允许程序员在没有操作系统的环境下编写嵌入式系统和底层代码。
```
### 插件合集
```shell
# 热重载插件
cargo install cargo-watch
# 监听改变
cargo watch -x run
# other
cargo watch -c -q -w ./src -x run
-c 来清空终端
-q 抑制cargo watch本身的输出
-w 关注某个目录，这里只关注src目录。
-x 运行cargo命令
```

```shell
# 命令行包管理工具
cargo install cargo-edit
```

```shell
# cargo modules插件允许我们可视化项目的模块结构，以树状格式显示模块结构。
cargo install cargo-modules
```

```shell
# cargo audit检查项目的依赖项是否有任何安全漏洞，这在持续集成中特别有用。
cargo install cargo-audit
```

```shell
# cargo tarpaulin 是另一个对持续集成非常有用的插件，这个插件计算项目的代码覆盖率。
cargo install cargo-tarpaulin
```

```shell
# cargo-nextest 是新一代的rust测试程序，它提供了漂亮的测试结果，片状的测试检测，并且在某些代码库上可以将测试运行速度提高60倍。
cargo install cargo-nextest
```

```shell
# cargo-make 是rust的任务运行器和构建工具，它允许你定义一组任务并在流程中运行它们，任务可以在toml文件中定义。
cargo install cargo-make
```

```shell
# cargo-machete是一个Cargo插件，用于从项目中删除未使用的依赖项
cargo install cargo-machete
cargo machete
```

```sh
# Testcontainers为项目在本地添加的基础设施进行测试可能相当棘手，Testcontainers的目标是通过提供一个开源框架来解决这个问题，该框架为你的应用程序提供本地轻量级容器，这些容器在使用后可以立即丢弃
cargo add testcontainers
```

```sh
# tokio-console是一个对于使用Tokio的Rust异步程序的调试器。
```

```sh
# cargo-flamegraph是一个用Rust编写生成火焰图的程序，火焰图是分布式请求跟踪的可视化，最初是用Perl编写的，但现在已经移植到Rust中。通过使用火焰图，可以更容易地看到错误(例如，竞争条件)可能来自何处，或者可能在何处出现内存泄漏，异常高延迟或难以通过常规日志调试的错误等问题，可以通过这种方式更容易地解决，因为你可以可视化的看到调用堆栈。
cargo install flamegraph
```
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
- cargo build                   可以构建项目
- cargo build --release  当项目最终准备好发布时，来优化编译项目。
- cargo run                      可以运行项目
- cargo test                     可以测试项目
- cargo doc                     可以为项目构建文档
- cargo publish               可以将库发布到 crates.io。
- cargo --version
- cargo publish -- workspace 也支持pnpm类似的功能了

### Rustup

1. Rust和LLVM的关系是怎样的？

2. Rustup中targets是什么，为什么可以安装多个？

3. Rust在windows上为什么需要安装Visual studio？

4. Rust工具链有哪些工具？

- rustup 是什么？

```sh
Rustup是 Rust 的官方工具链管理器，它提供了一种方便的方式来安装、管理和切换不同的Rust工具链版本。总的来说有如下能力：

1. 安装 Rust：

• rustup 允许你轻松地安装最新版本的 Rust，包括稳定版、beta 版和 nightly 版。 rustup install stable

2. 切换 Rust 工具链版本：

• 你可以使用 rustup default 命令切换默认的 Rust 版本。 rustup default stable

• 也可以在项目级别使用 .rust-version 文件指定特定的 Rust 版本。

3. 管理目标（Targets）：

• rustup 允许你安装不同的目标，以支持交叉编译和在不同的平台上运行 Rust 代码。 rustup target add <target>

• 列出已安装的目标： rustup target list

4. 升级 Rust 工具链版本：

• 使用 rustup update 命令可以升级已安装的 Rust 工具链版本。 rustup update

5. 卸载 Rust：

• rustup 允许你卸载 Rust，并清理相关的工具链和组件。 rustup self uninstall

6. 组件管理：

• rustup 允许你安装和管理不同的 Rust 组件，如 rust-src、rust-analysis 等。 rustup component add rust-src

7. 查看工具链信息：

• 使用 rustup show 命令可以查看有关当前 Rust 环境的详细信息，包括已安装的工具链、组件等。 rustup show

通过上面的内容我们知道了Rustup可以管理toolchain和target，那么toolchain和target究竟是什么呢?

```

- 工具链
```sh
toolchain指一组Rust工具，包括编译器（rustc）、构建工具（cargo）、文档生成工具（rustdoc）以及其他与 Rust 相关的实用程序。Toolchain用于管理和构建 Rust 代码，并且可以包括一个特定版本的 Rust 编译器和标准库，还包含一个默认是编译到本机平台的target。工具链的版本可以是 "stable"（稳定版）、"beta"（测试版）或 "nightly"（每日构建版），每个版本都对应着不同的 Rust 编译器和特性。

下面这些常用的命令可以操作工具链：

# 安装新的toolchain
rustup install stable
# 设置默认的toolchain
rustup default stable
# 列出已经安装的toolchain
rustup toolchain list
# 更新到最新稳定版
rustup update stable
# 更新到指定版本
rustup update <version>
# 显示toolchain和targets
rustup show
下面这些就是工具链中的工具命令了，它们通常存储在~/.cargo/bin这个目录下。

1. rustc: Rust编译器，负责将Rust源代码编译为机器码。它是Rust的主要编译器，也是构建Rust程序的关键组件。

2. Cargo: Rust的构建系统和包管理器。Cargo简化了项目的创建、依赖管理和构建过程。它还提供了一组命令用于构建、运行测试、发布和安装Rust程序。

3. rustdoc: Rust的文档生成工具。通过使用特定的注释格式，rustdoc能够生成漂亮的文档，帮助开发者编写和维护文档。

4. rustfmt: 代码格式化工具，用于保持Rust代码的一致性和可读性。它能够格式化代码，使其符合Rust语言的约定。

5. rustup: Rust的工具链管理器，用于安装、升级和管理Rust的不同版本。它还允许你切换默认的Rust版本，以适应项目的需求。

6. rls (Rust Language Server): 提供了与IDE（集成开发环境）集成所需的功能，例如代码补全、跳转到定义、查找引用等。支持的IDE包括Visual Studio Code、Atom等。

7. cargo-make: 用于创建和运行自定义构建任务的工具。它允许开发者在构建过程中执行自定义的命令和脚本。

8. miri: Rust的Mir Interpreter，用于执行和测试Rust程序在MIR（Mid-level Intermediate Representation）级别的代码。Miri有助于检测一些可能的内存安全问题。

这只是Rust工具链中的一部分工具。Rust社区积极发展和维护工具链，以提高开发者的工作效率，并确保Rust代码的质量和安全性。你可以通过查阅Rust官方文档或使用cargo --list命令查看完整的工具列表。

```

- 跨平台开发和交叉编译
```sh
Rust 中的target概念主要是为了支持跨平台开发和交叉编译，以确保 Rust 代码可以在不同的操作系统和架构上正确运行。Rustc target指的是编译和构建目标平台Rust代码时需要的组件。不要混淆为Rust项目编译后产生的target文件夹。它的格式表示为：<arch>-<vendor>-<sys>-<abi>。其中：

• <arch> 表示架构（例如，x86_64 表示 64 位的 x86 架构）。

• <vendor> 表示供应商（一般为空）。

• <sys> 表示操作系统（例如，linux、windows、macos 等）。

• <abi> 表示二进制接口（例如，默认的是 "gnu"，也可以是 "musl"、"msvc" 等）。

示例：

• x86_64-unknown-linux-gnu: 64位 x86 架构，Linux 操作系统。

• i686-pc-windows-msvc: 32位 x86 架构，Windows 操作系统，使用 MSVC 编译器。

• aarch64-apple-ios: 64位 ARM 架构，iOS 操作系统。

• wasm32-unknown-unknown: WebAssembly 目标。

一般来说只需要rustup target add 命令安装某个目标平台组件即可，但对于一些特殊平台，可能需要手动安装相关的交叉编译工具链，例如windows msvc或者android NDK。

下面是操作Target常用的命令：

# 列出可用的target
rustup target list
# 安装一个新的rustup target add <target>
rustup target add x86_64-unknown-linux-gnu
# 把代码编译到指定平台
cargo build --target x86_64-unknown-linux-gnu
• 安装新目标：rustup target add <target>

• 列出已安装的目标：rustup target list

我在初学rust时，就经常混淆toolchain和target。现在我们知道了target是toolchain的一部分，编译到特定目标平台要使用对应目标平台的target。
```

- Rust编译器中的LLVM、MSVC、GNU
```sh
你可能听说过Rust编译器后端使用了LLVM，那为何还需要msvc和gnu呢，为啥Go和Java这些语言不需要呢？那我们来理一下Rust编译过程你就清楚了。

Rust编译器实际上是由多个组件组成的，其中之一是“rustc”做为前端编译器，而编译器的后端使用了LLVM。

1. 前端编译器（rustc）： 这一部分负责将Rust源代码转换为中间表示（Intermediate Representation，IR），该表示形式在Rust中称为“MIR”（Mid-level Intermediate Representation）。

2. 后端编译器（LLVM）： MIR然后被传递给LLVM，LLVM是一个开源的编译器基础设施，提供了许多通用的优化和代码生成工具。LLVM将MIR转换为目标机器的机器码，并执行一系列优化，以生成最终的可执行文件。

以Windows平台为例，Rust编译器就是rustc.exe，它首先会把源码编译为MIR，然后交给LLVM处理，LLVM继续把MIR先编译成LLVM IR进而编译为目标平台的机器码（此时还不是执行文件，只是一堆机器码）。往后就是target发挥作用了，target调用msvc或gnu来完成链接步骤，主要是链接目标平台库文件并生成可执行文件。这里整个编译过程几乎都是由rustc.exe完成的，因为它包含了llvm和调用target的代码，跟目标平台相关的工作则是由msvc或gnu来完成。msvc和gnu是c/c++的编译工具链，编译后的最终产物就是可执行文件或库，rustc在编译后期用到了它们提供的功能。

整个编译过程大致如下：

Source code -> MIR -> LLVM IR -> 机器码 -> Target链接 -> 可执行文件或库

Go编译器是自己实现了链接目标平台的工作，因此不需要msvc或gnu。Java也是类似，所有的底层工作都是Java虚拟机实现的，javac仅仅是把源码编译成class字节码就结束了。
```

### VScode编辑器配置
https://code.visualstudio.com/docs/languages/rust

## 入门

### Hello Rust！
### 创建新项目
```shell
cargo new learning
# 创建git版本管理的仓库
cargo new learning --vcs=git
```
![](readme.assets/Pasted%20image%2020230715211915.png)
### 构建二进制文件

  单文件构建，不推荐使用。
```sh
rustc main.rs
```

这里才是最常用的项目构建，不同的系统，构建的可执行二进制文件不同。
```shell
cargo build
```
![](readme.assets/Pasted%20image%2020230715213137.png)
target目录解析

|文件夹/文件|内容和作用|
|---|---|
|`debug`|调试模式下的构建输出，包括可执行文件、依赖库和编译缓存。|
|`release`|发布模式下的构建输出，优化后的可执行文件和依赖库。|
|`build`|构建过程中生成的中间文件和临时文件。|
|`deps`|项目的依赖项的编译结果，用于链接到最终二进制文件。|
|`incremental`|增量编译的缓存数据，加速后续的编译过程。|
|`.fingerprint`|项目及其依赖项的指纹文件，跟踪文件变化决定是否重新编译。|
|`doc`|通过 `cargo doc` 生成的项目文档，HTML格式的API文档。|
|`examples`|项目中示例代码的编译结果，展示项目功能使用方法。|
|`package`|用于发布到 crates.io 的打包文件（通过 `cargo package` 生成）。|
|`tmp`|构建过程中创建的临时文件。|

### 执行主函数
![](readme.assets/Pasted%20image%2020230715213623.png)
有意思的地方来了。 这个方法不再推荐
```
rustc main.rs # 编译
./debug/learning # 执行
```
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
#### Setting基本配置
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
#### Launch调试
要开始调试，您首先需要安装具有调试支持的两种语言扩展之一：

- [Microsoft C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) (ms-vscode.cpptools) –_在 Windows 上_
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) (vadimcn.vscode-lldb) –_在 macOS/Linux 上_

如果您忘记安装这些扩展之一，当您尝试启动调试会话时，rust-analyzer 将提供一条通知，其中包含 VS Code Marketplace 的链接。

设置断点, 选择调试器。
![](readme.assets/Pasted%20image%2020230715222428.png)
![](readme.assets/Pasted%20image%2020230715222636.png)
```json
{
    // 使用 IntelliSense 了解相关属性。 
    // 悬停以查看现有属性的描述。
    // 欲了解更多信息，请访问: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            // windows上 使用 cppvsdbg
            "type": "cppvsdbg",
            // 附加
            "request": "launch",
            // 调试器名
            "name": "(windows)rustRun",
            // 一般执行代码都在这里src/main.rs，这个位置相对引用更好些
            // D:\RustCode\learning\target\debug\learning.exe
            "program": "${fileWorkspaceFolder}/${input:currentProjectName}/target/debug/${input:currentProjectName}.exe",
            // 用于查找依赖项和其他文件的当前工作目录
            "cwd": "${workspaceFolder}",
            // 使用vscode集成终端
            // "console": "integratedTerminal",
            // rs文件调试前，执行的任务
            "preLaunchTask": "build",
        },
        {
            // linux/mac上，记得使用 lldb
            "type": "lldb",
            // 附加
            "request": "launch",
            // 调试器名
            "name": "(OSX)rustRun",
            // 一般执行代码都在这里src/main.rs，这个位置相对引用更好些
            // D:\RustCode\learning\target\debug\learning.exe
            "program": "${fileWorkspaceFolder}/${input:currentProjectName}/target/debug/${input:currentProjectName}",
            // 用于查找依赖项和其他文件的当前工作目录
            "cwd": "${workspaceFolder}",
            // 使用vscode集成终端
            // "console": "integratedTerminal",
            // rs文件调试前，执行的任务
            "preLaunchTask": "build",
        }
    ],
    "inputs": [
        {
            // 用户输入id的内容，用在program
            "id": "currentProjectName",
            // PromptString：显示一个输入框以从用户处获取字符串。
            // pickString：显示快速选择下拉列表，让用户从多个选项中进行选择。
            // command：运行任意命令。
            "type": "promptString",
            // 提示输入
            "description": "选择你要调试可执行文件",
            // 默认是learning.exe
            "default": "learning",
            // 不隐藏
            "password": false
        },
    ]
}
```
#### Tasks构建任务
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            // 调试前，先构建一遍
            "label": "build",
            "type": "shell",
            "command": "cargo build",
            // 可能需要给 cargo build 赋予参数，
            "args": [],
            "options": {
                "cwd": "${fileDirname}"
            }
        },
    ],
}
```

### 教程版
跟随网络的教程开始学习。
https://www.rust-lang.org/zh-CN/learn
```
Rust 程序设计语言的本质实际在于 赋能（empowerment）：无论你现在编写的是何种代码，Rust 能让你在更为广泛的编程领域走得更远，写出自信。（这一点并不显而易见）

举例来说，那些“系统层面”的工作涉及内存管理、数据表示和并发等底层细节。从传统角度来看，这是一个神秘的编程领域，只为浸润多年的极少数人所触及，也只有他们能避开那些臭名昭著的陷阱。即使谨慎的实践者，亦唯恐代码出现漏洞、崩溃或损坏。

Rust 破除了这些障碍：它消除了旧的陷阱，并提供了伴你一路同行的友好、精良的工具。想要 “深入” 底层控制的程序员可以使用 Rust，无需时刻担心出现崩溃或安全漏洞，也无需因为工具链不靠谱而被迫去了解其中的细节。更妙的是，语言设计本身会自然而然地引导你编写出可靠的代码，并且运行速度和内存使用上都十分高效。

已经在从事编写底层代码的程序员可以使用 Rust 来提升信心。例如，在 Rust 中引入并行是相对低风险的操作，因为编译器会替你找到经典的错误。同时你可以自信地采取更加激进的优化，而不会意外引入崩溃或漏洞。

但 Rust 并不局限于底层系统编程。它表达力强、写起来舒适，让人能够轻松地编写出命令行应用、网络服务器等各种类型的代码——在本书中就有这两者的简单示例。使用 Rust 能让你把在一个领域中学习的技能延伸到另一个领域：你可以通过编写网页应用来学习 Rust，接着将同样的技能应用到你的 Raspberry Pi（树莓派）上。
```
使用rust构建稳健可靠的底层系统
```
Rust 也为系统编程世界带来了现代化的开发工具：

Cargo 是内置的依赖管理器和构建工具，它能轻松增加、编译和管理依赖，并使依赖在 Rust 生态系统中保持一致。
Rustfmt 格式化工具确保开发者遵循一致的代码风格。
Rust Language Server 为集成开发环境（IDE）提供了强大的代码补全和内联错误信息功能。
通过使用 Rust 生态系统中丰富的工具，开发者可以更高效地编写系统层面代码。
```
#### 猜字游戏
main.rs
```rust
// 导入库

use ferris_says::say;

use std::io::{stdout, BufWriter};

// 导入自己包，先声明同级目录下的rs文件

// 私有模块声明

mod game;

// 共有模块声明

// pub mod game;

  

// 这是相对路径

use game::{g, test};

// 只引用一个函数

// use game::g;

// 绝对路径引入，这是根路径

// use crate::game::test;

  

fn test_says() {

let stdout: std::io::Stdout = stdout();

let message: &str = "Hello fellow Rustaceans!";

let width: usize = message.chars().count();

  

let mut writer: BufWriter<std::io::StdoutLock<'_>> = BufWriter::new(stdout.lock());

say(message, width, &mut writer).unwrap();

}

  

// !主程序入口函数

fn main() {

// 打印

// println!("Hello, Rust!");

// 测试函数

// test_says();

// 测试导包

g();

}

```
game.rs
```rust
// 当前子模块参数，对象，方法等全暴露给其他模块

// pub(crate)

  

// 标准库std，引入其中的io模块

use std::io;

// 引入三方随机数库

use rand::Rng;

// 引入比较库

use std::cmp::Ordering;

  

// 只暴露这个函数

pub fn test() {

println!("find me！");

// 创建变量

let apples: i32 = 5; // 不可变

let mut bananas: i32 = 5; // 可变

  

// 模版字符串

let x = 5;

let y = 10;

  

println!("x = {x} and y + 2 = {}", y + 2);

}

  

pub fn g() {

println!("Guess the number!");

  

// !随机数生成，再控制个范围，链式调用，太熟悉了

let secret_number = rand::thread_rng().gen_range(1..=100);

  

// println!("The secret number is: {secret_number}");

  

// 再套入循环

loop {

println!("Please input your guess.");

// 创建一个 变量（variable）来储存用户输入，创建一个new string的空字符串实例

  

let mut guess = String::new();

  

// 写法一，如果你没有引用io，则标准库可以在这里引用

// std::io::stdin()

// .read_line(&mut guess)

// .expect("Failed to read line");

  

// 写法二

io::stdin()

// 我们还将 &mut guess 作为参数传递给 read_line() 函数，让其将用户输入储存到这个字符串中。并且read_line只能追加，不能覆盖

.read_line(&mut guess) // 调用read_line方法从标准输入

.expect("Failed to read line"); // !

/*

todo 这里就是所有权了，也是rust的核心

* & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数* * 据，而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势* 就是安全而简单的操纵引用。完成当前程序并不需要了解如此多细节。现在，我* 们只需知道它像变量一样，默认是不可变的。因此，需要写成 &mut guess 来* 使其可变，而不是 &guess。

*/

  

/*

todo expect捕捉异常

read_line 会将用户输入附加到传递给它的字符串中，不过它也会返回一个类型为 Result 的值。 Result 是一种枚举类型，通常也写作 enum。枚举类型变量的值可以是多种可能状态中的一个。我们把每种可能的状态称为一种 枚举成员（variant）。

*/

  

/*

Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且包含失败的前因后果。

  

这些 Result 类型的作用是编码错误处理信息。Result 类型的值，像其他类型一样，拥有定义于其上的方法。Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。在本例中，这个值是用户输入到标准输入中的字节数。

  

如果不调用 expect，程序也能编译，不过会出现一个警告：

  
  

$ cargo build

Compiling guessing_game v0.1.0 (file:///projects/guessing_game)

warning: unused `Result` that must be used

--> src/main.rs:10:5

|

10 | io::stdin().read_line(&mut guess);

| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

|

= note: `#[warn(unused_must_use)]` on by default

= note: this `Result` may be an `Err` variant, which should be handled

  

warning: `guessing_game` (bin "guessing_game") generated 1 warning

Finished dev [unoptimized + debuginfo] target(s) in 0.59s

Rust 警告我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理。

  

消除警告的正确做法是实际去编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect。

*/

  

// 模版字符串语法

// println!("You guessed: {guess}");

  

// 类型转换语法

/*

创建了一个叫做 guess 的变量。不过等等，不是已经有了一个叫做 guess 的变量了吗？确实如此，不过 Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值。这个功能常用在需要转换值类型之类的场景。

  

String 实例的 trim 方法会去除字符串开头和结尾的空白字符，我们必须执行此方法才能将字符串与 u32 比较，因为 u32 只能包含数值型数据。

  

字符串的 parse 方法 将字符串转换成其他类型。这里用它来把字符串转换为数值。我们需要告诉 Rust 具体的数字类型，这里通过 let guess: u32 指定。guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型。Rust 有一些内建的数字类型；u32 是一个无符号的 32 位整型。

*/

// * 语法糖，分别处理成功or失败

let guess: u32 = match guess.trim().parse() {

Ok(num) => num,

Err(_) => continue,

};

  

// 开始比较, 但是必须类型一致，也就说我们必须转换字符串到u32

  

match guess.cmp(&secret_number) {

Ordering::Less => println!("too small"),

Ordering::Greater => println!("too big"),

Ordering::Equal => {

// 这个箭头函数真的强

println!("you win");

break;

}

}

}

}

```
- 编译并执行
win32平台
```
# windows会编译成为.exe文件
$ cargo build
# 执行方式一
$ cargo run
# 执行方式二
$ ./package.exe
```
macOS/Linux平台
```
# windows会编译成为二进制文件
$ cargo build
# 执行方式一
$ cargo run
# 执行方式二
$ ./package
```
**如果你更熟悉动态语言，如 Ruby、Python 或 JavaScript，则可能不习惯将编译和运行分为两个单独的步骤。
Rust 是一种 预编译静态类型（ahead-of-time compiled）语言，这意味着你可以编译程序，并将可执行文件送给其他人，他们甚至不需要安装 Rust 就可以运行。如果你给他人一个 .rb、.py 或 .js 文件，他们需要先分别安装 Ruby，Python，JavaScript 实现（运行时环境，VM）。**
不过在这些语言中，只需要一句命令就可以编译和运行程序。这一切都是语言设计上的权衡取舍。
仅仅使用 rustc 编译简单程序是没问题的，不过随着项目的增长，你可能需要管理你项目的方方面面，并让代码易于分享。接下来，我们要介绍一个叫做 Cargo 的工具，它会帮助你编写真实世界中的 Rust 程序。

#### 模块化和包管理
Cargo 会给我们的包创建一个 Cargo.toml 文件。查看 Cargo.toml 的内容，会发现并没有提到 src/main.rs，因为 Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

在此，我们有了一个只包含 src/main.rs 的包，意味着它只含有一个名为 my-project 的二进制 crate。如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个二进制的和一个库的，且名字都与包相同。通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
- 从 crate 根节点开始: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码
<div style="color: red;">ceate表示当前文件夹的根文件，根节点</div>
- 声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
	- 内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
	- 在文件 src/garden.rs
	- 在文件 src/garden/mod.rs
<div style="color: red;">有两种声明模块的语法</div>
- 声明子模块: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在src/garden.rs中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：
	- 内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
	- 在文件 src/garden/vegetables.rs
	- 在文件 src/garden/vegetables/mod.rs
<div style="color: red;">子模块本质是文件夹层级问题。</div>
- 模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在crate::garden::vegetables::Asparagus被找到。
<div style="color: red;">如果你想暴漏一个file.rs中对象，还需要pub 和 pub(ceate)</div>
- 私有 vs 公用: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。
<div style="color: red;">凡是pub则任何模块都可以嵌套调用。</div>
- use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过 use crate::garden::vegetables::Asparagus;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。
<div style="color: red;">
<div>绝对路径引用：use crate::garden::vegetables::Asparagus; 带create</div>
<div>相对路径引用：use garden::vegetables::Asparagus; 不带create</div>
</div>
#### 基本变量和概念
- 字面量：= 字面量 
	- 就是等号右边的就是字面量
- 变量：mut
- 常量：const
	- 如果不加mut，都默认是常量。


#### 数据类型

| 类型名称                                                | 是否为内建类型 | 所属类别   | 简要用途说明                                                                                                                                       | 示例用法                                                                                                      |
| --------------------------------------------------- | ------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| **有符号整数**`i8`, `i16`, `i32`, `i64`, `i128`, `isize` | 是       | 基本类型   | 有符号整数类型，可表示正负整数。按位宽区分有 8位到128位 (`i8`~`i128`)，以及依赖平台指针宽度的 `isize`。用于整数运算和计数索引等。                                                               | `let x: i32 = -42;`                                                                                       |
| **无符号整数**`u8`, `u16`, `u32`, `u64`, `u128`, `usize` | 是       | 基本类型   | 无符号整数类型，只能表示非负整数。包括 8位到128位 (`u8`~`u128`)，以及依赖平台的 `usize`。常用于计数、内存大小等不能为负的值。                                                                 | `let y: u8 = 255;`                                                                                        |
| **浮点数**`f32`, `f64`                                 | 是       | 基本类型   | 浮点数类型，表示实数（带小数）。提供32位单精度 `f32` 和64位双精度 `f64` 两种。默认类型为 `f64`，用于科学计算、坐标等连续值。                                                                   | `let z: f64 = 3.14;`                                                                                      |
| **字符**`char`                                        | 是       | 基本类型   | 字符类型，表示一个 Unicode 标量值，大小为四个字节。用于存储单个字符，例如字母、汉字或表情符号。                                                                                         | `let c: char = '中';`                                                                                      |
| **布尔**`bool`                                        | 是       | 基本类型   | 布尔类型，值为 `true` 或 `false`。用于逻辑判断和条件控制。                                                                                                        | `let b: bool = true;`                                                                                     |
| **单元类型**`()`                                        | 是       | 基本类型   | 单元类型，表示“空”的值类型（空元组）。通常用作占位符，函数不返回值时其返回类型为 `()`。                                                                                              | `let unit: () = ();`                                                                                      |
| **Never 类型**`!`                                     | 是       | 基本类型   | **永不返回**的特殊类型，表示不存在值（用于发散函数）。函数返回 `!` 意味着函数不会正常返回（例如调用 `panic!` 或死循环）。                                                                       | `fn abort() -> ! { panic!("错误"); }`                                                                       |
| **元组**_Tuple_                                       | 是       | 复合类型   | 将多个值组合为一个复合类型。元组长度固定，内部元素可以是不同类型。常用于将相关的数据打包成一个整体返回或传递。                                                                                      | `let tup: (i32, f64, bool) = (42, 3.14, true);`                                                           |
| **数组**_Array_ `[T; N]`                              | 是       | 复合类型   | 定长的序列，每个元素类型相同。数组的长度在编译期确定，数据存储在栈上。适用于存放固定大小的同类数据。                                                                                           | `let arr: [i32; 3] = [1, 2, 3];`                                                                          |
| **切片**_Slice_ `[T]` （如 `&[T]`）                      | 是       | 复合类型   | 动态长度的序列视图，对数组或向量等连续元素序列的引用。切片不拥有数据，仅指向现有集合的一部分，可变长度。在运行时记录长度，常以引用形式出现（例如字符串切片 `&str`）。                                                       | `let arr = [10, 20, 30]; let slice: &[i32] = &arr[0..2];`                                                 |
| **字符串**`String`                                     | 否       | 所有权类型  | 提供 **堆** 上存储的可变长字符串，拥有所含文本数据。`String` 是 UTF-8 编码，可增长、修改，适合需要所有权的文本数据。                                                                        | `let s: String = String::from("Hello");`                                                                  |
| **向量**`Vec<T>`                                      | 否       | 所有权类型  | 可增长的动态数组，存储在堆上。`Vec<T>` 可在运行时动态增加或减少长度，适合存放可变数量的同类型数据。                                                                                       | `let v: Vec<i32> = vec![1, 2, 3];`                                                                        |
| **Box 智能指针**`Box<T>`                                | 否       | 所有权类型  | 在堆上分配值的指针，提供对单个数据的所有权。`Box<T>` 允许将一个值存于堆内，栈上持有指向堆数据的指针。常用于创建递归类型或将大型数据移至堆上。                                                                  | `let b: Box<i32> = Box::new(5);`                                                                          |
| **可选值**`Option<T>`                                  | 否       | 所有权类型  | 可选值枚举类型：要么是 `Some(T)` 包含一个值，要么是 `None` 表示无值。用于表示一个值可能存在或缺失，避免空指针。                                                                            | `let maybe: Option<i32> = None;`                                                                          |
| **结果类型**`Result<T, E>`                              | 否       | 所有权类型  | 错误处理枚举：`Result` 包含两种变体，`Ok(T)` 表示成功并含返回值，`Err(E)` 表示失败并含错误信息。用于传播和处理可恢复错误。                                                                   | `let res: Result<i32, &str> = Err("错误");`                                                                 |
| **引用**`&T`（共享引用），`&mut T`（可变引用）                     | 是       | 引用类型   | 安全引用类型，对值的借用指针。`&T` 允许只读访问，`&mut T` 允许独占可变访问。引用不取得所有权，确保数据借用期间有效且遵循借用规则。                                                                     | `let mut x = 5; let y: &mut i32 = &mut x;`                                                                |
| **原始指针**`*const T`，`*mut T`                         | 是       | 指针类型   | 未经安全检查的裸指针，可用于与 C 交互或底层操作。`*const T` 指向常量数据，`*mut T` 指向可变数据。原始指针不受借用规则保护，可能出现空指针或悬垂指针风险。                                                     | `let x = 10; let p: *const i32 = &x as *const i32;`                                                       |
| **函数指针**`fn` 类型                                     | 是       | 函数类型   | 函数的指针类型，指向可调用的函数代码。具有固定的参数和返回类型签名，如 `fn(i32) -> i32`。可将函数当作值传递或存储，用于回调等场景。                                                                   | `fn add(x: i32, y: i32) -> i32 { x+y } let f: fn(i32, i32) -> i32 = add;`                                 |
| **闭包**_Closure_（实现 `Fn` 系列 trait）                   | 是       | 函数类型   | 匿名函数，可以捕获环境变量。每个闭包有自己唯一的类型，实现 `Fn`/`FnMut`/`FnOnce` trait，可像函数一样调用。常用于将行为作为参数传递。                                                             | `let inc =                                                                                                |
| **结构体**_struct_                                     | 否       | 高级抽象类型 | 用户自定义复合类型，通过字段将多个相关值组合为一个对象。可以为结构体定义方法和实现 trait。用于表示具有属性的实体。                                                                                 | `struct Point { x: i32, y: i32 }; let p = Point { x: 1, y: 2 };`                                          |
| **枚举**_enum_                                        | 否       | 高级抽象类型 | 用户自定义枚举类型，定义一组有限可能的变体，每个变体可附带数据。用于表示状态或不同类型的值集合（如 `Option`、`Result` 也是枚举）。                                                                   | `enum Color { Red, Green, Blue } let c = Color::Red;`                                                     |
| **Trait 对象**`dyn Trait`                             | 否       | 高级抽象类型 | 特征对象，通过关键字 `dyn` 表示的动态分发类型。允许在运行时通过 trait 接口操作不同具体类型，实现多态。`dyn Trait` 类型在编译期大小不确定，运行时通过虚表调用。通常以引用或指针形式使用（如 `&dyn Trait` 或 `Box<dyn Trait>`）。 | `trait Trait { fn foo(&self); } struct S; impl Trait for S { fn foo(&self){} } let obj: &dyn Trait = &S;` |
| **哈希映射**`HashMap<K, V>`                             | 否       | 集合类型   | 键值对集合，基于哈希表实现。通过键快速查找对应的值，键可以是实现了 `Eq` 和 `Hash` 的类型。插入、查找和移除平均性能接近 O(1)。                                                                     | `use std::collections::HashMap; let mut map = HashMap::new(); map.insert("a", 1);`                        |
| **哈希集合**`HashSet<T>`                                | 否       | 集合类型   | 不含重复元素的无序集合，基于哈希表。支持快速判断元素是否存在。适用于需要一组唯一值的场景。                                                                                                | `use std::collections::HashSet; let mut set = HashSet::new(); set.insert(5);`                             |
| **B 树映射**`BTreeMap<K, V>`                           | 否       | 集合类型   | 键值对有序集合，基于 B 树实现。按键排序存储数据，支持有序遍历和范围查询。适用于需要按顺序迭代或范围查找的映射。                                                                                    | `use std::collections::BTreeMap; let mut bt = BTreeMap::new(); bt.insert(1, "a");`                        |
| **引用计数指针**`Rc<T>`                                   | 否       | 智能指针   | 单线程引用计数智能指针，实现多所有者共享同一数据。通过引用计数跟踪堆上的对象，最后一个持有者被丢弃时自动释放内存。只能用于单线程环境。                                                                          | `use std::rc::Rc; let a = Rc::new(5); let b = Rc::clone(&a);`                                             |
| **原子引用计数指针**`Arc<T>`                                | 否       | 智能指针   | 原子引用计数智能指针，提供线程安全的共享所有权。`Arc` 使用原子操作更新计数，可用于多线程环境下安全地共享数据。适合跨线程传递不可变数据。                                                                      | `use std::sync::Arc; let a = Arc::new(5); let b = Arc::clone(&a);`                                        |
| **内部可变容器**`Cell<T>`                                 | 否       | 内部可变性  | 提供内部可变性的容器类型。`Cell` 允许在不需要可变引用的情况下修改值（通过复制语义），适用于实现不可变结构体中隐藏可变状态。                                                                            | `use std::cell::Cell; let c = Cell::new(5); c.set(10); let x = c.get();`                                  |
| **可变借用检查器**`RefCell<T>`                             | 否       | 内部可变性  | 提供运行时借用检查的可变容器。`RefCell` 允许在不可变上下文中执行可变借用（通过 `borrow`/`borrow_mut`），在运行时确保借用规则，违规时触发 panic。用于单线程场景下的内部可变性。                                   | `use std::cell::RefCell; let c = RefCell::new(5); *c.borrow_mut() = 10;`                                  |
| **互斥锁**`Mutex<T>`                                   | 否       | 并发锁    | 提供线程间独占访问的锁。一次仅允许一个线程获取锁并访问内部数据，确保共享可变数据的并发安全。常用于保护临界区，内部数据需通过 `lock()` 获取。                                                                  | `use std::sync::Mutex; let m = Mutex::new(0); *m.lock().unwrap() = 1;`                                    |
| **读写锁**`RwLock<T>`                                  | 否       | 并发锁    | 提供多读者/单写者的并发锁。允许多个线程同时读取或独占写入。适用于读多写少场景，提高并发性能。读取使用 `read()` 获取共享锁，写入使用 `write()` 获取独占锁。                                                     | `use std::sync::RwLock; let rw = RwLock::new(5); *rw.write().unwrap() = 10;`                              |


##### 变量
```rust
// 变量mut

pub fn test_mut() {

let mut x = 5;

println!("The value of x is: {x}");

x = 6;

println!("The value of x is: {x}");

}

  

// 常量, 无法变更，const

pub fn constants() {

const THREE_HOURS_INSECONDS: u32 = 60 * 60 * 3;

println!("{}", THREE_HOURS_INSECONDS)

}

  

// 隐藏，当定义域不同时，同一个变量改变时，会发生不同的结果。

pub fn Shadowing() {

let x: i32 = 5;

  

let x = x + 1;

  

{

let x = x * 2;

println!("The value of x in the inner scope is: {x}");

}

  

println!("The value of x is: {x}");

}

  

// 变量类型的强制性

pub fn testType() {

// todo 这种可以修改已命名的值

let spaces = " ";

let spaces = spaces.len();

// todo 这种就会报错，虽然你可以修改value，但不允许修改它的type。

// let mut spaces = " ";

// spaces = spaces.len();

}
```

##### 标量
![](readme.assets/Pasted%20image%2020230718225406.png)
![](readme.assets/Pasted%20image%2020230718225808.png)
```rust
/*

!标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。

长度 有符号 无符号

8-bit i8 u8

16-bit i16 u16

32-bit i32 u32

64-bit i64 u64

128-bit i128 u128

arch isize usize

  

数字字面值 例子

Decimal (十进制) 98_222

Hex (十六进制) 0xff

Octal (八进制) 0o77

Binary (二进制) 0b1111_0000

Byte (单字节字符)(仅限于u8) b'A'

  

* 整型溢出，是无法通过编译的。

* 使用 --release flag 在 release 模式中构建时，Rust 不会检测会导致 panic 的整型溢出。相反发生整型溢出时，Rust 会进行一种被称为二进制补码 wrapping（two’s complement wrapping）的操作。

*/

// 使用整型

pub fn changeType() {

let guess: u32 = "42".parse().expect("Not a number!");

println!("{}", guess)

}

  

// 浮点数，默认都是双精度，单精度也能用

pub fn float() {

let x = 2.0; // f64

let y: f32 = 3.0; // f32

}

  

// 数值计算

pub fn count() {

// addition

let sum = 5 + 10;

  

// subtraction

let difference = 95.5 - 4.3;

  

// multiplication

let product = 4 * 30;

  

// division

let quotient = 56.7 / 32.2;

let truncated = -5 / 3; // 结果为 -1

  

// remainder

let remainder = 43 % 5;

}

  

// 布尔值

pub fn bools() {

let t = true;

  

let f: bool = false; // with explicit type annotation

}

  

// 字符类型，这个很特殊

pub fn string() {

let c = 'z';

let z: char = 'ℤ'; // with explicit type annotation

let heart_eyed_cat = '😻';

}

  

// 复合类型

pub fn tuple() {

let tup = (500, 6.4, 1);

  

// 解构赋值

let (x, y, z) = tup;

  

// 点索引赋值

// let five_hundred = x.0;

  

// let six_point_four = x.1;

  

// let one = x.2;

  

println!("The value of y is: {y}");

}

  

pub fn array() {

let months = [

"January",

"February",

"March",

"April",

"May",

"June",

"July",

"August",

"September",

"October",

"November",

"December",

];

// 直接给每个元素赋予type

let a: [i32; 5] = [1, 2, 3, 4, 5];

  

// 分号表示五个元素，五个元素为3

let a = [3; 5];

  
  

// 通过索引访问数组元素

let first = a[0];

let second = a[1];

  
  

}

  

use std::io;

  
  

// 通过输入索引，获取数组中的value。如果超过索引范围，则报错

fn test() {

let a = [1, 2, 3, 4, 5];

  

println!("Please enter an array index.");

  

let mut index = String::new();

  

io::stdin()

.read_line(&mut index)

.expect("Failed to read line");

  

let index: usize = index

.trim()

.parse()

.expect("Index entered was not a number");

  

let element = a[index];

  

println!("The value of the element at index {index} is: {element}");

}
```
##### 函数
```rust
// 函数

  

/*

Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。

*/

  

// 有点python的类型注解的感觉了

pub fn print_labeled_measurement(value: i32, unit_label: char) {

println!("The measurement is: {value}{unit_label}");

}

  

// 表达式写法

pub fn talk() {

let y = {

let x = 3;

x + 1

};

  

println!("The value of y is: {y}");

}

  
  

// 返回值函数，只需要箭头函数

fn plus_one(x: i32) -> i32 {

// 不能加入分号，“mismatched types”（类型不匹配）

x + 1

}
```
##### 分支结构（判断，循环）
```rust
/*

!值得注意的是代码中的条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。例如，尝试运行以下代码

*/

pub fn testIf() {

let number = 3;

  

if number % 4 == 0 {

println!("number is divisible by 4");

} else if number % 3 == 0 {

println!("number is divisible by 3");

} else if number % 2 == 0 {

println!("number is divisible by 2");

} else {

println!("number is not divisible by 4, 3, or 2");

}

}

  

pub fn testIf_2() {

let condition = true;

// 语法糖, 但是返回值的类型必须一致

let number = if condition { 5 } else { 6 };

  

println!("The value of number is: {number}");

}

  

pub fn lp() {

// 当运行这个程序时，我们会看到连续的反复打印 again!，直到我们手动停止程序。大部分终端都支持一个快捷键，ctrl-c，来终止一个陷入无限循环的程序。

// loop {

// println!("again!");

// }

  

let mut counter = 0;

  

let result = loop {

counter += 1;

  

if counter == 10 {

break counter * 2;

}

};

  

println!("The result is {result}");

}

  

// 循环嵌套

pub fn lp2() {

let mut count = 0;

// 加入标签，指示跳出的是哪层循环

'counting_up: loop {

println!("count = {count}");

let mut remaining = 10;

  

loop {

println!("remaining = {remaining}");

if remaining == 9 {

break;

}

if count == 2 {

break 'counting_up;

}

remaining -= 1;

}

  

count += 1;

}

println!("End count = {count}");

}

  

// while 条件循环

fn lp3() {

let a = [10, 20, 30, 40, 50];

let mut index = 0;

  

while index < 5 {

println!("the value is: {}", a[index]);

  

index += 1;

}

}

  

// !for循环 最好用的循环

fn lp4() {

let a = [10, 20, 30, 40, 50];

  

for element in a {

println!("the value is: {element}");

}

  

// rev翻转字符串，1..4是一个数组的语法糖

for number in (1..4).rev() {

println!("{number}!");

}

println!("LIFTOFF!!!");

}
```

#### ⭐️所有权
所有权（系统）是 Rust 最为与众不同的特性，对语言的其他部分有着深刻含义。<span style="color: red;">它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全，因此理解 Rust 中所有权如何工作是十分重要的。</span>本章，我们将讲到所有权以及相关功能：借用（borrowing）、slice 以及 Rust 如何在内存中布局数据。

Rust 的核心功能（之一）是 所有权（ownership）。虽然该功能很容易解释，但它对语言的其他部分有着深刻的影响。
所有程序都必须管理其运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。<p style="color: green">Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。</p>
因为所有权对很多程序员来说都是一个新概念，需要一些时间来适应。好消息是随着你对 Rust 和所有权系统的规则越来越有经验，你就越能自然地编写出安全和高效的代码。持之以恒！

举例：
```
入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。

访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）。继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。

当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的主要目的就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。
```
#####  所有权规则
```
1.Rust 中的每一个值都有一个 所有者（owner）。
2.值在任一时刻有且只有一个所有者。
3.当所有者（变量）离开作用域，这个值将被丢弃。
```

##### 内存与分配
就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：

必须在运行时向内存分配器（memory allocator）请求内存。
需要一个当我们处理完 String 时将内存返回给分配器的方法。
第一部分由我们完成：当调用 String::from 时，它的实现 (implementation) 请求其所需的内存。这在编程语言中是非常通用的。

然而，第二部分实现起来就各有区别了。在有 垃圾回收（garbage collector，GC）的语言中，GC 记录并清除不再使用的内存，而我们并不需要关心它。在大部分没有 GC 的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。我们需要精确的为一个 allocate 配对一个 free。

Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。下面是示例 4-1 中作用域例子的一个使用 String 而不是字符串字面值的版本：
```rust
   {
        let s = String::from("hello"); // 从此处起，s 是有效的

        // 使用 s
    }                                  // 此作用域已结束，
                                       // s 不再有效
```
 
这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

注意：在 C++ 中，这种 item 在生命周期结束时释放资源的模式有时被称作 资源获取即初始化（Resource Acquisition Is Initialization (RAII)）。如果你使用过 RAII 模式的话应该对 Rust 的 drop 函数并不陌生。

这个模式对编写 Rust 代码的方式有着深远的影响。现在它看起来很简单，不过在更复杂的场景下代码的行为可能是不可预测的，比如当有多个变量使用在堆上分配的内存时。现在让我们探索一些这样的场景。

```rust
pub fn scope() {

// 函数作用域

/*

当 s 进入作用域 时，它就是有效的。

这一直持续到它 离开作用域 为止。

*/

let s = "函数外作用域";

{

// 内部属性和方法，只能内部调用，除非你给返回值

let s = "函数作用域字符串";

}

  

/*

这是一个将 String 需要的内存返回给分配器的很自然的位置：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。

*/

  

println!("{}", s)

}

  

pub fn scope1() {

// *使用from函数基于字符串的字面值来创建string

// *:这两个冒号 :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。

let mut s = String::from("前\n");

  

s.push_str("追加字符串"); // push_str() 在字符串后追加字面值

  

println!("{}", s);

  

/*

?就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。不过这些特性都只得益于字符串字面值的不可变性。不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

  

*/

}

  

pub fn scope2() {

// 变量和数据交互的方式：这个会让rust深复制。

let x = 5;

let y = x;

  

// 方式二：这种则会让rust潜复制，让s2指向s1点内存地址。

let s1 = String::from("hello");

let s2 = s1;

  

/*

todo 当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

  

todo rust的解决思路：s1被清理，s2彻底取代s1。

*/

  

// s2正常使用

// println!("{s2}");

// s1 则报错error[E0382]: borrow of moved value: `s1`

// println!("{s1}")

}

  

pub fn scope3() {

// 深复制，只针对字面量from

let s1 = String::from("hello");

let s2 = s1.clone();

  

println!("s1 = {}, s2 = {}", s1, s2);

  

// 拷贝, 这里没有使用字面量from，

let x = 5;

let y = x;

  

println!("x = {}, y = {}", x, y);

}

  

pub fn scope4() {

let s = String::from("hello"); // s 进入作用域

  

takes_ownership(s); // s 的值移动到函数里 ...

// ... 所以到这里不再有效

  

let x = 5; // x 进入作用域

  

makes_copy(x); // x 应该移动函数里，

// 但 i32 是 Copy 的，

// 所以在后面可继续使用 x

  

let response = gives_ownership();

println!("{response}");

  

// jie解构赋值

let str1 = String::from("abc");

let (s2, s2lenght) = calculate_length(str1);

println!("{s2}, {s2lenght}")

}

  

fn takes_ownership(some_string: String) {

// some_string 进入作用域

println!("{}", some_string);

} // 这里，some_string 移出作用域并调用 `drop` 方法。

// 占用的内存被释放

  

fn makes_copy(some_integer: i32) {

// some_integer 进入作用域

println!("{}", some_integer);

} // 这里，some_integer 移出作用域。没有特殊之处

  

fn gives_ownership() -> String {

// gives_ownership 会将

// 返回值移动给

// 调用它的函数

  

let some_string = String::from("yours"); // some_string 进入作用域。

  

some_string // 返回 some_string

// 并移出给调用的函数

//

}

  

// 返回一个元祖

fn calculate_length(s: String) -> (String, usize) {

let length = s.len(); // len() 返回字符串的长度

(s, length)

}
```
##### 引用与借用
``` rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```
我们不能在同一时间多次将 s 作为可变变量借用。第一个可变的借入在 r1 中，并且必须持续到在 println！ 中使用它，但是在那个可变引用的创建和它的使用之间，我们又尝试在 r2 中创建另一个可变引用，该引用借用与 r1 相同的数据。

这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用。新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
- 两个或更多指针同时访问同一数据。
- 至少有一个指针被用来写入数据。
- 没有同步数据访问的机制。

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有：
```rust
   let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
```

Rust 在同时使用可变与不可变引用时也采用的类似的规则。这些代码会导致一个错误：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // 大问题

    println!("{}, {}, and {}", r1, r2, r3);
}

```
哇哦！我们 也 不能在拥有不可变引用的同时拥有可变引用。
不可变引用的用户可不希望在他们的眼皮底下值就被意外的改变了！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。
注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的：
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

```
不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。它们的作用域没有重叠，所以代码是可以编译的。编译器可以在作用域结束之前判断不再使用的引用。

尽管这些错误有时使人沮丧，但请牢记这是 Rust 编译器在提前指出一个潜在的 bug（在编译时而不是在运行时）并精准显示问题所在。这样你就不必去跟踪为何数据并不是你想象中的那样。

###### 引用的规则

- 在任意给定时间，
	- 要么 只能有一个可变引用，
	- 要么 只能有多个不可变引用。
- 引用必须总是有效的。

##### Slice类型
slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。

```rust
/*

todo 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 与指针不同，引用确保指向某个特定类型的有效值。

*/

  

// 形参也要注明引用

fn calculate_length(s: &String) -> usize {

s.len()

}

  

fn change(some_string: &mut String) {

some_string.push_str(", world");

}

  

// 不支持返回引用值, 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。

// fn dangle() -> &String {

// let s = String::from("hello");

// &s

// }

  

// 获取首字母

fn first_word(s: &String) -> usize {

// 因为需要逐个元素的检查 String 中的值是否为空格，需要用 as_bytes 方法将 String 转化为字节数组

let bytes = s.as_bytes();

// 接下来，使用 iter 方法在字节数组上创建一个迭代器：

for (i, &item) in bytes.iter().enumerate() {

if item == b' ' {

// i是索引，item是元素

return i;

}

}

  

// 如果没有空格，则整个字符串就是首字母

s.len()

}

  

// 获取首单词

fn first_word2(s: &str) -> &str {

let bytes = s.as_bytes();

for (i, &item) in bytes.iter().enumerate() {

if item == b' ' {

return &s[0..i];

}

}

&s[..]

}

  

pub fn demo() {

let mut s1 = String::from("hello");

// !使用 & 即可引用。

let len = calculate_length(&s1);

// println!("The length of '{}' is {}.", s1, len);

  

// 修改引用的值，则需要 &mut 将传入的参数转换为可变引用.

change(&mut s1);

// println!("{s1}"); // hello, world

  

// 同一个定义域下，不支持两次可变引用的再赋值。

let r1 = &mut s1;

// let r2 = &mut s1; // error[E0499]: cannot borrow `s` as mutable more than once at a time

// println!("{r1}");

  

// 悬垂指针（dangling pointer）

// let test = dangle(); // error[E0106]: missing lifetime specifier

  

// 迭代器分割字符串

let mut data = String::from("ab c");

// let mut res = first_word(&data);

// data.clear(); // data会被清空

// println!("{res}, {data}"); // 这里不会报错

  

// 改进

let res2 = first_word2(&data);

// data.clear();

println!("{res2}"); // 这里由于是引用分割，当你清空原始对象后，会报错。error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable

  

// 字符串切割

let s = String::from("hello world");

// let hello = &s[0..5];

let hello = &s[..5];

let endIndex = s.len();

// let world = &s[6..11];

let world = &s[6..endIndex];

  

// 获取整个字符串

// let slice = &s[0..len];

// let slice = &s[..];

  

// println!("{s}, {hello}, {world}")

// let x = &s[-1]; // 不支持负索引

  

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

// 读写

for s in slice {

println!("{}", s);

}

// 断言相等

assert_eq!(slice, &[2, 3]);

  
  

}
```

#### ⭐️结构体struct
struct，或者 structure，是一个自定义数据类型，允许你包装和命名多个相关的值，从而形成一个有意义的组合。如果你熟悉一门面向对象语言，struct 就像对象中的数据属性。在本章中，我们会对元组和结构体进行比较和对比。

生命周期确保结构体引用的数据有效性跟结构体本身保持一致。
```rust
struct User {
    active: bool,
    // 造成生命周期无效
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

##### 方法语法
为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写），这个 impl 块中的所有内容都将与 Rectangle 类型相关联。接着将 area 函数移动到 impl 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 self。然后在 main 中将我们先前调用 area 方法并传递 rect1 作为参数的地方，改成使用 方法语法（method syntax）在 Rectangle 实例上调用 area 方法。方法语法获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。

在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。在一个 impl 块中，Self 类型是 impl 块的类型的别名。方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写。注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我们在 rectangle: &Rectangle 中做的那样。方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。

这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。

使用方法替代函数，除了可使用方法语法和不需要在每个函数签名中重复 self 的类型之外，其主要好处在于组织性。我们将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找 Rectangle 的功能。
```rust

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。

它是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```
第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。

```rust
// 定义结构体

struct User {

active: bool,

username: String,

email: String,

sign_in_count: i32,

}

  

fn build_user(email: String, username: String) -> User {

/*

构建结构体

*/

User {

active: true,

username: username,

email: email,

sign_in_count: 1,

}

}

  

// 元组结构体

struct Color(i32, i32, i32);

  

// 类单元结构体

struct AlwaysEqual;

  

pub fn theWorld() {

// 有点对象语法的感觉

let mut user1 = User {

active: true,

username: String::from("someusername123"),

email: String::from("someone@example.com"),

sign_in_count: 1,

};

// 结构体赋值

user1.email = String::from("785632486@qq.com");

// println!("{}", user1.email);

  

// 使用user1的内容，填充给user2

let user2 = User {

active: user1.active,

username: user1.username.clone(),

email: String::from("another@example.com"),

sign_in_count: user1.sign_in_count,

};

// 使用语法糖，进行解构赋值

let user3 = User {

email: String::from("another@example.com"),

// 使用结构体语法糖

..user1

};

// user1被清理掉了，user3能正常使用。

// println!("{}", user1.email);

// println!("{}", user2.email);

// println!("{}", user3.sign_in_count);

// clone 可以进行属性的深复制, 但结构体不能深复制

let user4 = user3.email.clone();

// println!("{}", user4);

  

// 元组结构体

let black = Color(0, 0, 0);

  

// 类单元结构体, 本质就是unit类型 结构体 类似 空对象

let subject = AlwaysEqual;

  

// 结构体传参数

// wh();

  

// 结构体原型，方法

// print();

  

// 结构体参数增加

// inherit();

  

// !关联函数, 涉及到模块语法

testSelf();

}

  

// 使用 #[derive(Debug)] 后才能打印结构体

#[derive(Debug)]

struct Rectangle {

width: u32,

height: u32,

}

  

// 计算矩形面积

fn wh() {

let width1 = 30;

let height1 = 50;

println!("{}", area(width1, height1));

  

let rect1 = (30, 50);

println!("{}", area1(rect1));

  

let rect2 = Rectangle {

width: 30,

height: 50,

};

  
  
  

println!("{}", area2(&rect2));

// println!("{rect2:#?}");

// println!("{rect2:?} \n dbg!(&rect2)");

// dbg! 一个简单的宏tips，不受#[derive(Debug)]干涉，随时可用

dbg!(&rect1);

}

  

fn area(width: u32, height: u32) -> u32 {

width * height

}

  

fn area1(dimensions: (u32, u32)) -> u32 {

// 使用元组结构体，计算

dimensions.0 * dimensions.1

}

  

fn area2(rectangle: &Rectangle) -> u32 {

// 使用普通对象结构体

rectangle.height * rectangle.width

}

  

// 为结构体添加方法，这里很像js的prototype，高级对象方法。

impl Rectangle {

// &self 代表结构体本身，类似js的this

fn area(&self) -> u32 {

// 这个self又类似

self.width * self.height

}

// 调用内部方法的返回值，类似计算属性的引用，

fn width(&self) -> bool {

self.width > 0

}

}

  

// 通过派生 trait 增加实用功能, 打印结构体

fn print() {

let rect1 = Rectangle {

width: 30,

height: 50,

};

// 非格式化语法

// println!("rect1 is {:?}", rect1);

// 格式化语法

// println!("rect1 is {:#?}", rect1);

  

// 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。

  

// 打印到异常控制台

// dbg!(&rect1);

  

// 调用结构体内部方法

println!("{}", rect1.area());

if rect1.width() {

println!("宽度要大于0， 则当前值是 {}", rect1.width);

}

}

  

impl Rectangle {

// 添加新的结构体方法

fn can_hold(&self, other: &Rectangle) -> bool {

self.width > other.width && self.height > other.height

}

}

  

// 结构体参数修改

fn inherit() {

let rect1 = Rectangle {

width: 30,

height: 50,

};

let rect2 = Rectangle {

width: 10,

height: 40,

};

let rect3 = Rectangle {

width: 60,

height: 45,

};

  

println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

  

// 结构体的关联函数

impl Rectangle {

// 这里就涉及到模块了

fn square(size: u32) -> Self {

Self {

width: size,

height: size,

// 不能增删参数， 必须和结构体保持一致

// long: size,

}

}

}

  

fn testSelf() {

let sq = Rectangle::square(23);

println!("{:#?}", sq)

}
```
结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。

但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。

#### ⭐️枚举和模式
本章介绍 枚举（enumerations），也被称作 enums。枚举允许你通过列举可能的 成员（variants）来定义一个类型。首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。接下来，我们会探索一个特别有用的枚举，叫做 Option，它代表一个值要么是某个值要么什么都不是。然后会讲到在 match 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。最后会介绍 if let，另一个简洁方便处理代码中枚举的结构。

```rust
/*

枚举结构体

  

结构体给予你将字段和数据聚合在一起的方法，像 Rectangle 结构体有 width 和 height 两个字段。而枚举给予你将一个值成为一个集合之一的方法。比如，我们想让 Rectangle 是一些形状的集合，包含 Circle 和 Triangle 。为了做到这个，Rust 提供了枚举类型。

  

让我们看看一个需要诉诸于代码的场景，来考虑为何此时使用枚举更为合适且实用。假设我们要处理 IP 地址。目前被广泛使用的两个主要 IP 标准：IPv4（version four）和 IPv6（version six）。这是我们的程序可能会遇到的所有可能的 IP 地址类型：所以可以 枚举 出所有可能的值，这也正是此枚举名字的由来。

  

任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。IP 地址的这个特性使得枚举数据结构非常适合这个场景，因为枚举值只可能是其中一个成员。IPv4 和 IPv6 从根本上讲仍是 IP 地址，所以当代码在处理适用于任何类型的 IP 地址的场景时应该把它们当作相同的类型。

  

可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：

*/

  

pub fn enums() {

#[derive(Debug)]

enum IpAddrKind {

V4,

V6,

}

// 分别导出enum的两个实例

// 注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开

let four = IpAddrKind::V4;

let six = IpAddrKind::V6;

  

// 使用成员调用函数

route(IpAddrKind::V4);

route(IpAddrKind::V6);

  

fn route(ip_kind: IpAddrKind) {

println!("{:#?}", ip_kind)

}

}

  

// 枚举字段，赋予类型

  

pub fn enums1() {

#[derive(Debug)]

enum IpAddr {

V4(String),

V6(String),

}

  

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

println!("{:#?}", home);

  

// * 直接构建元组

enum IpAddr1 {

V4(u8, u8, u8, u8),

V6(String),

}

  

let home = IpAddr1::V4(127, 0, 0, 1);

let loopback = IpAddr1::V6(String::from("::1"));

  

// todo 结构体，枚举

struct Ipv4Addr {

// --snip--

b: i32,

}

  

struct Ipv6Addr {

// --snip--

a: String,

}

  

enum IpAddr2 {

V4(Ipv4Addr),

V6(Ipv6Addr),

}

  

// ?枚举类型

enum Message {

// 类单元结构体

Quit,

// 普通结构体

Move { x: i32, y: i32 },

// 元组结构体

Write(String),

// 元组结构体

ChangeColor(i32, i32, i32),

}

  

// 枚举也是一种结构

impl Message {

fn call(&self) {

// 在这里定义方法体

}

}

  

let m = Message::Write(String::from("hello"));

m.call();

}

  

pub fn enums2() {

enum Option<T> {

// 空值

None,

// 任意数据，泛型

Some(T),

}

  

let some_number = Some(5);

let some_char = Some('e');

let absent_number: Option<i32> = Option::None;

}

  

pub fn enums3() {

enum Coin {

Penny,

Nickel,

Dime,

Quarter,

}

fn value_in_cents(coin: Coin) -> u8 {

match coin {

Coin::Penny => {

println!("Lucky penny!");

1

}

Coin::Nickel => 5,

Coin::Dime => 10,

Coin::Quarter => 25,

}

}

// 返回1

println!("{}", value_in_cents(Coin::Penny));

  

fn plus_one(x: Option<i32>) -> Option<i32> {

match x {

None => None,

Some(i) => Some(i + 1),

}

}

  

let five = Some(5);

let six = plus_one(five);

let none = plus_one(None);

}

  

pub fn enums4() {

fn plus_one(x: Option<i32>) -> Option<i32> {

match x {

Some(i) => Some(i + 1),

// 要求必须穷尽所有的可能，包括none

None => None,

}

}

  

// todo other通配符

let dice_roll = 9;

match dice_roll {

3 => 3,

7 => 7,

// other 可以获取match传递下来的值

other => other,

// 不要match传递下来的值

// _ => reroll(),

};

  

// if let 简单判断, 语法糖

let config_max = Some(3u8);

if let Some(max) = config_max {

println!("The maximum is configured to be {}", max);

}

  

// 语法糖2

let mut count = 0;

// match coin {

// Coin::Quarter(state) => println!("State quarter from {:?}!", state),

// _ => count += 1,

// }

  

// 语法糖3

let mut count = 0;

// if let Coin::Quarter(state) = coin {

// println!("State quarter from {:?}!", state);

// } else {

// count += 1;

// }

}
```
一种特殊的结构体。比价适合判断。

#### 包，库，creat模块管理
对于一个由一系列相互关联的包组成的超大型项目，Cargo 提供了 “工作空间” 这一功能， “Cargo Workspaces” 。

讨论封装来实现细节，这可以使你更高级地重用代码：你实现了一个操作后，其他的代码可以通过该代码的公共接口来进行调用，而不需要知道它是如何实现的。你在编写代码时可以定义哪些部分是其他代码可以使用的公共部分，以及哪些部分是你有权更改实现细节的私有部分。这是另一种减少你在脑海中记住项目内容数量的方法。

这里有一个需要说明的概念 “作用域（scope）”：代码所在的嵌套上下文有一组定义为 “in scope” 的名称。当阅读、编写和编译代码时，程序员和编译器需要知道特定位置的特定名称是否引用了变量、函数、结构体、枚举、模块、常量或者其他有意义的项。你可以创建作用域，以及改变哪些名称在作用域内还是作用域外。同一个作用域内不能拥有两个相同名称的项；可以使用一些工具来解决名称冲突。
Rust 有许多功能可以让你管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能。这有时被称为 “模块系统（the module system）”，包括：

- 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates ：一个模块的树形结构，它形成了库或二进制项目。
- 模块（Modules）和 use：允许你控制作用域和路径的私有性。
- 路径（path）：一个命名例如结构体、函数或模块等项的方式。

##### 创建包
```shell
$ cargo new my-project
```
目录结构
![](readme.assets/Pasted%20image%2020230731234223.png)
##### 定义模块
模块系统的部分，如允许你命名项的 路径（paths）；用来将路径引入作用域的 use 关键字；以及使项变为公有的 pub 关键字。我们还将讨论 as 关键字、外部包和 glob 运算符。
```
从 crate 根节点开始: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。

声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
在文件 src/garden.rs
在文件 src/garden/mod.rs

声明子模块: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在src/garden.rs中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：
内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
在文件 src/garden/vegetables.rs
在文件 src/garden/vegetables/mod.rs

模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在crate::garden::vegetables::Asparagus被找到。

私有 vs 公用: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。

use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过 use crate::garden::vegetables::Asparagus;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。
```

###### 创建库
```shell
cargo new --lib restaurant
```
src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。
![](readme.assets/Pasted%20image%2020230801001058.png)
路径有两种形式：
- 绝对路径（absolute path）是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于对于当前 crate 的代码，则以字面值 crate 开头。
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

```rust
// crate 是当前项目的根文件, 对于一个二进制 crate 而言是src/main.rs

// gardan的代码路径

use crate::garden::vegetables::Asparagus;

// 申明子模块的路径

pub mod garden;

  

fn main() {

let plant = Asparagus {};

println!("I'm growing {:?}!", plant);

my::indirect_call();

}

  
  

fn function() {

println!("called `function()`");

}

  

mod cool {

pub fn function() {

println!("called `cool::function()`");

}

}

  

mod my {

fn function() {

println!("called `my::function()`");

}

mod cool {

pub fn function() {

println!("called `my::cool::function()`");

}

}

pub fn indirect_call() {

// 让我们从这个作用域中访问所有名为 `function` 的函数！

print!("called `my::indirect_call()`, that\n> ");

// `self` 关键字表示当前的模块作用域——在这个例子是 `my`。

// 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，

// 因为他们表示相同的函数。

self::function();

function();

// 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：

self::cool::function();

// `super` 关键字表示父作用域（在 `my` 模块外面）。

super::function();

// 这将在 *crate* 作用域内绑定 `cool::function` 。

// 在这个例子中，crate 作用域是最外面的作用域。

{

use crate::cool::function as root_function;

root_function();

}

}

}
```
##### 使用共有结构体
```rust
mod back_of_house {

// 共有结构体

pub struct Breakfast {

pub toast: String,

seasonal_fruit: String,

}

  

impl Breakfast {

pub fn summer(toast: &str) -> Breakfast {

Breakfast {

toast: String::from(toast),

seasonal_fruit: String::from("peaches"),

}

}

}

}

  

pub fn eat_at_restaurant() {

// 在夏天订购一个黑麦土司作为早餐

let mut meal = back_of_house::Breakfast::summer("Rye");

// 改变主意更换想要面包的类型

meal.toast = String::from("Wheat");

println!("I'd like {} toast please", meal.toast);

  

// 如果取消下一行的注释代码不能编译；

// 不允许查看或修改早餐附带的季节水果

// meal.seasonal_fruit = String::from("blueberries");

}
```
```rust
mod back_of_house {

// 共有枚举

pub enum Appetizer {

Soup,

Salad,

}

}

  

pub fn eat_at_restaurant() {

let order1 = back_of_house::Appetizer::Soup;

let order2 = back_of_house::Appetizer::Salad;

}

```
##### 使用use
```rust
// 使用use将模块引入作用域

  

mod front_of_house {

pub mod hosting {

pub fn add_to_waitlist() {}

}

}

  

use crate::front_of_house::hosting;

  

pub fn eat_at_restaurant() {

hosting::add_to_waitlist();

}
```
```rust
mod front_of_house {

pub mod hosting {

pub fn add_to_waitlist() {}

}

}

  

use crate::front_of_house::hosting;

  

mod customer {

// 编译器错误显示短路径不在适用于 customer 模块中：

// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

hosting::add_to_waitlist();

}

}
```
```rust
use std::fmt;

use std::io;

// 如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。

fn function1() -> fmt::Result {

// --snip--

Ok(())

}

  

fn function2() -> io::Result<()> {

// --snip--

Ok(())

}

  
  

// 另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。示例

use std::collections::HashMap;

fn test() {

let mut map = HashMap::new();

map.insert(1, 2);

}

  

// 正常引用

mod front_of_house {

pub mod hosting {

pub fn add_to_waitlist() {}

}

}

  

use crate::front_of_house::hosting::add_to_waitlist;

  

pub fn eat_at_restaurant() {

add_to_waitlist();

}
```
##### 使用as
```rust
use std::fmt::Result;

use std::io::Result as IoResult;

  

fn function1() -> Result {

// --snip--

Ok(())

}

  

fn function2() -> IoResult<()> {

// --snip--

Ok(())

}
```
##### 二次封装
```rust
mod front_of_house {

pub mod hosting {

pub fn add_to_waitlist() {}

}

}

  

pub use crate::front_of_house::hosting;

  

pub fn eat_at_restaurant() {

hosting::add_to_waitlist();

}
```
##### 使用三方库
![](readme.assets/Pasted%20image%2020230801005250.png)
![](readme.assets/Pasted%20image%2020230801005354.png)
##### 引用一个库的多个函数
![](readme.assets/Pasted%20image%2020230801005700.png)
##### 将模块拆分成多个文件
经验问题。
![](readme.assets/Pasted%20image%2020230801010244.png)
#### 常见集合
Rust 标准库中包含一系列被称为 集合（collections）的非常有用的数据结构。大部分其他数据类型都代表一个特定的值，不过集合可以包含多个值。不同于内建的数组和元组类型，这些集合指向的数据是储存在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小。每种集合都有着不同功能和成本，而根据当前情况选择合适的集合，这是一项应当逐渐掌握的技能。在这一章里，我们将详细的了解三个在 Rust 程序中被广泛使用的集合：

- vector 允许我们一个挨着一个地储存一系列数量可变的值
- 字符串（string）是字符的集合。我们之前见过 String 类型，不过在本章我们将深入了解。
- 哈希 map（hash map）允许我们将值与一个特定的键（key）相关联。这是一个叫做 map 的更通用的数据结构的特定实现。

##### 列表vector
```rust
pub fn call() {

// 创建存储列表，数据结构

// let mut v: Vec<i32> = Vec::new();

// 简写方式

// let v = vec![1, 2, 3];

let mut v = vec![];

// 开始堆栈

v.push(5);

v.push(6);

v.push(7);

v.push(8);

println!("{:?}", v);

  

// 引用

let third: &i32 = &v[2];

println!("The third element is {third}");

  

// 获取

let third: Option<&i32> = v.get(2);

match third {

Some(third) => println!("The third element is {third}"),

None => println!("There is no third element."),

}

  

// let does_not_exist = &v[100];

// println!("{}", does_not_exist);

  

// get 可以避免索引报错。取不到的值，用none代替

let does_not_exist = v.get(100);

println!("{:?}", does_not_exist);

}

  

pub fn call2() {

// 已经不会报错了

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

}

  

pub fn call3() {

let v = vec![100, 32, 57];

// 循环引用

for i in &v {

println!("{i}");

}

  

// 循环改写

let mut v = vec![100, 32, 57];

for i in &mut v {

*i += 50;

}

  

// 枚举结构自适应

enum SpreadsheetCell {

Int(i32),

Float(f64),

Text(String),

}

  

let row = vec![

SpreadsheetCell::Int(3),

SpreadsheetCell::Text(String::from("blue")),

SpreadsheetCell::Float(10.12),

];

}
```

##### string字符串
在开始深入这些方面之前，我们需要讨论一下术语 字符串 的具体意义。Rust 的核心语言中只有一种字符串类型：字符串 slice str，它通常以被借用的形式出现，&str。第四章讲到了 字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。
```rust
pub fn strMain() {

// 空字符串

let mut s = String::new();

  

// 不可变字符串

let data = "initial contents";

  

// 转字符串

let s = data.to_string();

  

// 该方法也可直接用于字符串字面值：

let s = "initial contents".to_string();

  

// 创建字符串

let s = String::from("initial contents");

  

// 追加字符串 的 几种方式

let mut s = String::from("foo");

s.push_str("bar");

s += "baz";

s.push('l');

println!("{}", s);

  

// 使用format!

let s1 = String::from("tic");

let s2 = String::from("tac");

let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");

println!("{}", s);

  

// !索引字符串？根本不支持的。

// * 1.字符串长度在内存中不是字面量的长度

// * 2.let hello = "Здравствуйте";let answer = &hello[0];

// let hello = "Здравствуйте";

// let answer = &hello[0];

  

// 遍历字符串

for c in "Зд".chars() {

println!("{c}");

}

for b in "Зд".bytes() {

println!("{b}");

}

  

/*

标准库提供了很多围绕 String 和 &str 构建的功能，来帮助我们正确处理这些复杂场景。请务必查看这些使用方法的文档，例如 contains 来搜索一个字符串，和 replace 将字符串的一部分替换为另一个字符串。

  

称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。当 Rustacean 们谈到 Rust 的 “字符串”时，它们通常指的是 String 或字符串 slice &str 类型，而不特指其中某一个。

*/

}
```
##### hash map 存储键值对
常用集合类型是 哈希 map（hash map）。HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
```rust
use std::collections::HashMap;

  

pub fn hashmaps() {

let mut scores = HashMap::new();

// 写入值

scores.insert(String::from("Blue"), 10);

scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {

println!("{key}: {value}");

}

// 访问值

let team_name = String::from("Blue");

let score = scores.get(&team_name).copied().unwrap_or(0);

  

// 插入hash

let field_name = String::from("Favorite color");

let field_value = String::from("Blue");

let mut map = HashMap::new();

// field_name， field_name 立刻失效

map.insert(field_name, field_value);

  

// 覆盖更新hash

scores.insert(String::from("Blue"), 10);

scores.insert(String::from("Blue"), 25);

// 检查更新，没有才增加

scores.entry(String::from("Yellow")).or_insert(50);

scores.entry(String::from("Blue")).or_insert(50);

  

// 根据旧值更新新值

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {

let count = map.entry(word).or_insert(0);

*count += 1;

}

println!("{:?}", map);

}
```

#### ⭐️错误处理
错误是软件中不可否认的事实，所以 Rust 有一些处理出错情况的特性。在许多情况下，Rust 要求你承认错误的可能性，并在你的代码编译前采取一些行动。这一要求使你的程序更加健壮，因为它可以确保你在将代码部署到生产环境之前就能发现错误并进行适当的处理。

Rust 将错误分为两大类：
可恢复的（recoverable）
不可恢复的（unrecoverable）错误。
对于一个可恢复的错误，比如文件未找到的错误，我们很可能只想向用户报告问题并重试操作。
不可恢复的错误总是 bug 出现的征兆，比如试图访问一个超过数组末端的位置，因此我们要立即停止程序。

大多数语言并不区分这两种错误，并采用类似异常这样方式统一处理他们。Rust 没有异常。相反，它有 Result<T, E> 类型，用于处理可恢复的错误，还有 panic! 宏，在程序遇到不可恢复的错误时停止执行。
##### 不可恢复错误panic
修改配置cargo.toml，开启终止。
在任意rs中写入panic。
![](readme.assets/Pasted%20image%2020230805153742.png)
##### 可恢复错误result
主要是利用枚举，穷尽异常。
```rust
let greeting_file_result = File::open("hello.txt");

let greeting_file = match greeting_file_result {

Ok(file) => file,

// 这里则是通用异常

// Err(error) => panic!("Problem opening the file: {:?}", error),

// 枚举再细化，区分不同的错误

Err(error) => match error.kind() {

// 找不到文件，则创建文件

ErrorKind::NotFound => match File::create("hello.txt") {

Ok(fc) => fc,

Err(e) => panic!("Problem creating the file: {:?}", e),

},

// 其他异常就爆出

other_error => {

panic!("Problem opening the file: {:?}", other_error);

}

},

};
```
###### 使用闭包后更好的写法
```rust
let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {

if error.kind() == ErrorKind::NotFound {

File::create("hello.txt").unwrap_or_else(|error| {

panic!("Problem creating the file: {:?}", error);

})

} else {

panic!("Problem opening the file: {:?}", error);

}

});
```

###### 错误传播
标准格式
```rust
// 传播 propagating

fn read_username_from_file() -> Result<String, io::Error> {

let username_file_result = File::open("hello.txt");

// 打开文件

let mut username_file = match username_file_result {

Ok(file) => file,

Err(e) => return Err(e),

};

// 读取文件内容

let mut username = String::new();

match username_file.read_to_string(&mut username) {

Ok(_) => Ok(username),

Err(e) => Err(e),

}

}
```

简写格式。
```rust
fn read_username_from_file2() -> Result<String, io::Error> {

let mut username_file = File::open("hello.txt")?;

let mut username = String::new();

username_file.read_to_string(&mut username)?;

Ok(username)

}
```
再次简写
```rust
fn read_username_from_file3() -> Result<String, io::Error> {

let mut username = String::new();

File::open("hello.txt")?.read_to_string(&mut username)?;

Ok(username)

}
```
再次简写
```rust
// 再次简写

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
```
##### 主函数捕获异常
```rust
// crate 是当前项目的根文件, 对于一个二进制 crate 而言是src/main.rs

// gardan的代码路径

// use crate::garden::vegetables::Asparagus;

// 申明子模块的路径

// mod garden;

  

// 调用fs文件系统

use std::fs::File;

// use std::io::ErrorKind;

use std::error::Error;

// 调用异常捕获

mod err;

use err::demo;

  

fn main() -> Result<(), Box<dyn Error>> {

// !模块路径知识学习完毕

// let plant = Asparagus {};

// println!("I'm growing {:?}!", plant);

// my::indirect_call();

  

// !这里我们学习panic 不可恢复错误

// panic!("crash and burn");

/*

这里尝试访问 vector 的第一百个元素（这里的索引是 99 因为索引从 0 开始），不过它只有三个元素。这种情况下 Rust 会 panic。[] 应当返回一个元素，不过如果传递了一个无效索引，就没有可供 Rust 返回的正确的元素。

  

*C 语言中，尝试读取数据结构之后的值是未定义行为（undefined behavior）。你会得到任何对应数据结构中这个元素的内存位置的值，甚至是这些内存并不属于这个数据结构的情况。这被称为 缓冲区溢出（buffer overread），并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取储存在数据结构之后不被允许的数据。

  

为了保护程序远离这类漏洞，如果尝试读取一个索引不存在的元素，Rust 会停止执行并拒绝继续。尝试运行上面的程序会出现如下：

*/

// let v = vec![1, 2, 3, 4];

// v[99];

// ? shell执行 RUST_BACKTRACE=1 cargo run 即可获取更详细的报错内容

  

// !处理result，可恢复错误

// ?本质是利用了枚举成功or失败

// enum Result<T, E> {

// Ok(T),

// Err(E),

// }

// 打开文件

// let greeting_file_result = File::open("hello.txt");

// let greeting_file = match greeting_file_result {

// Ok(file) => file,

// // 这里则是通用异常

// // Err(error) => panic!("Problem opening the file: {:?}", error),

// // 枚举再细化，区分不同的错误

// Err(error) => match error.kind() {

// // 找不到文件，则创建文件

// ErrorKind::NotFound => match File::create("hello.txt") {

// Ok(fc) => fc,

// Err(e) => panic!("Problem creating the file: {:?}", e),

// },

// // 其他异常就爆出

// other_error => {

// panic!("Problem opening the file: {:?}", other_error);

// }

// },

// };

  

// * 更好的写法，利用闭包

// 利用链式调用

// let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {

// if error.kind() == ErrorKind::NotFound {

// File::create("hello.txt").unwrap_or_else(|error| {

// panic!("Problem creating the file: {:?}", error);

// })

// } else {

// panic!("Problem opening the file: {:?}", error);

// }

// });

  

// ? 链式语法，类似promise then cache的异常处理方法

// let greeting_file = File::open("hello.txt").unwrap();

// let greeting_file =

// File::open("hello.txt").expect("hello.txt should be included in this project");

  

// ? 这里引用err.rs中的其他api

demo();

  

// todo 哪怕是主函数，也可以进行错误传递

let greeting_file_result = File::open("hello.txt")?;

Ok(())

}

  

fn function() {

println!("called `function()`");

}

  

// module cool 模块cool

mod cool {

pub fn function() {

println!("called `cool::function()`");

}

}

  

// 模块my

mod my {

fn function() {

println!("called `my::function()`");

}

  

mod cool {

pub fn function() {

println!("called `my::cool::function()`");

}

}

  

pub fn indirect_call() {

// 让我们从这个作用域中访问所有名为 `function` 的函数！

print!("called `my::indirect_call()`, that\n> ");

  

// `self` 关键字表示当前的模块作用域——在这个例子是 `my`。

// 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，

// 因为他们表示相同的函数。

self::function();

function();

  

// 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：

self::cool::function();

  

// `super` 关键字表示父作用域（在 `my` 模块外面）。

super::function();

  

// 这将在 *crate* 作用域内绑定 `cool::function` 。

// 在这个例子中，crate 作用域是最外面的作用域。

{

use crate::cool::function as root_function;

root_function();

}

}

}

  

// 单个引用

// use std::cmp::Ordering;

// 引用上一级

// use std::io;

// 嵌套引用

// use std::{cmp::Ordering, io};

// 单个引用

// use std::io::Write;

// 括号批量引用

// use std::io::{self, Write};

// 全部引用

// use std::collections::*;
```
#### 泛型、Trait、生命周期
在 Rust 中其工具之一就是 泛型（generics）。泛型是具体类型或其他属性的抽象替代。我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。
trait，这是一个定义泛型行为的方法。trait 可以与泛型结合来将泛型限制为只接受拥有特定行为的类型，而不是任意类型。
最后介绍 生命周期（lifetimes），它是一类允许我们向编译器提供引用如何相互关联的泛型。Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。
###### 泛型T
```rust
fn type_of<T>(_: T) -> &'static str {

// 通过泛型，获取类型

type_name::<T>()

}

  

fn largest_i32(list: &[i32]) -> &i32 {

let mut largest = &list[0];

for item in list {

if item > largest {

largest = item;

}

}

largest

}

fn largest_char(list: &[char]) -> &char {

let mut largest = &list[0];

for item in list {

if item > largest {

largest = item;

}

}

largest

}

  

// fn largest<T>(list: &[T]) -> &T {

// let mut largest = &list[0];

// for item in list {

// if item > largest {

// largest = item;

// }

// }

// largest

// }

fn main() {
// !泛型

// let number_list = vec![34, 50, 25, 100, 65];

// let mut largest = &number_list[0];

// for number in &number_list {

// if number > largest {

// largest = number;

// }

// }

// println!("The largest number is {}", largest);

// * 抽象层

// let number_list = vec![34, 50, 25, 100, 65];

// let result = largest_i32(&number_list);

// println!("The largest number is {}", result);

// let char_list = vec!['y', 'm', 'a', 'q'];

// let result = largest_char(&char_list);

// println!("The largest char is {}", result);

// * 传入泛型，由输入形参决定类型

// let number_list = vec![34, 50, 25, 100, 65];

// let result = largest(&number_list);

// println!("The largest number is {}", result);

  

// *结构体中泛型

// struct Point<T> {

// x: T,

// y: T,

// };

// #[derive(Debug)]

// struct Point1<T, U> {

// x: T,

// y: U,

// }

// impl<T, U> Point1<T, U> {

// fn x(&self) -> &T {

// &self.x

// }

// }

// 强制制定泛型

// impl Point<f32> {

// fn distance_from_origin(&self) -> f32 {

// (self.x.powi(2) + self.y.powi(2)).sqrt()

// }

  

// }

// let wont_work = Point { x: 0, y: 4.0 }; // 会报错，类型不定

// let wont_work = Point1 { x: 0, y: 4.0 }; // 编译通过，

// println!("{:?}", wont_work);

// * 枚举泛型

// enum Option<T> {

// Some(T),

// None,

// }

// enum Result<T, E> {

// Ok(T),

// Err(E),

// }

// * 泛型嵌套

// struct Point<X1, Y1> {

// x: X1,

// y: Y1,

// }

  

// impl<X1, Y1> Point<X1, Y1> {

// fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {

// Point {

// x: self.x,

// y: other.y,

// }

// }

// }

// let p1 = Point { x: 5, y: 10.4 };

// let p2 = Point { x: "Hello", y: 'c' };

// let p3 = p1.mixup(p2);

// println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

  

// let integer = Some(5);

// let float = Some(5.0);

// *Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// let integer = Some(5);

// let float = Some(5.0);

// println!("{:?} is {}", integer, type_of(integer));

// println!("{:?} is {}", integer, type_of(float));Ï
}
```
###### 接口trait
trait 定义了某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

```rust
/*

! 这里放入 interfaces 接口

*/

pub trait Summary {

fn summarize(&self) -> String;

fn summarize_author(&self) -> String;

fn summarize_test(&self) -> String {

format!("调用我自己函数 {}", self.summarize())

}

}

pub trait Summary2 {

fn summarize(&self) -> String {

String::from("(Read more...)")

}

}

  

// 结构体

pub struct NewsArticle {

pub headline: String,

pub location: String,

pub author: String,

pub content: String,

}

// 引入接口详细方法

impl Summary2 for NewsArticle {}

  

// * 约束了 机构体的方法，

pub struct Tweet {

pub username: String,

pub content: String,

pub reply: bool,

pub retweet: bool,

}

  

// 引入接口模糊方法

impl Summary for Tweet {

fn summarize(&self) -> String {

format!("{}: {}", self.username, self.content)

}

fn summarize_author(&self) -> String {

format!("@{}", self.username)

}

}

  

// todo 标准格式化 接口

use std::fmt::{Debug, Display, Error, Formatter, Result};

// 多接口使用

impl Display for Tweet {

fn fmt(&self, f: &mut Formatter) -> Result {

write!(f, "{}", self.reply)

}

}

  

// *接受结构对象，跨作用域引用其他结构体的方法

// pub fn notify(item: &impl Summary) {

// println!("为其他 {}", item.summarize())

// }

  

// Trait Bound 语法糖

// pub fn notify<T: Summary>(item: &T) {

// println!("Breaking news! {}", item.summarize());

// }

  

// pub fn notify(item1: &impl Summary, item2: &impl Summary2) {

// println!("Breaking1 news! {}", item1.summarize());

// println!("Breaking2 news! {}", item2.summarize());

// }

  

// pub fn notify<T: Summary, U: Summary2>(item1: &T, item2: &U) {

// println!("Breaking1 news! {}", item1.summarize());

// println!("Breaking2 news! {}", item2.summarize());

// }

  

// * 多接口使用

// pub fn notify(item: &(impl Summary + Display)) {

// println!("Breaking news! {}", item.summarize());

// println!("news! {}", item);

// }

  

// pub fn notify<T: Summary + Display>(item: &T) {

// println!("Breaking news! {}", item.summarize());

// println!("news! {}", item);

// }

  

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// println!("难受")

// }

// todo 推荐写法

// fn some_function1<T, U>(t: &T, u: &U) -> i32

// where

// T: Display + Clone,

// U: Clone + Debug,

// {

// println!("Breaking news! {}", item.summarize());

// }

  

// !在返回值上定义

fn returns_summarizable() -> impl Summary {

Tweet {

username: String::from("??"),

content: String::from("xx"),

reply: false,

retweet: false,

}

}

  

pub fn interfaces() {

// todo 引入模糊方法

let tweet = Tweet {

username: String::from("horse_ebooks"),

content: String::from("of course, as you probably already know, people"),

reply: false,

retweet: false,

};

  

// println!("引入模糊方法: {}", tweet.summarize());

// println!("引入模糊方法: {}", tweet.summarize_author());

// println!("{}", tweet.summarize_test());

  

// todo 引入详细方法

let article = NewsArticle {

headline: String::from("Penguins win the Stanley Cup Championship!"),

location: String::from("Pittsburgh, PA, USA"),

author: String::from("Iceburgh"),

content: String::from(

"The Pittsburgh Penguins once again are the best \

hockey team in the NHL.",

),

};

// println!("引入详细方法: {}", article.summarize());

// *避开作用域问题，调用给其他函数引用

// notify(&tweet);

// notify(&tweet, &article);

// some_function(&tweet, &article)

// some_function1(&tweet, &article)

// let D = returns_summarizable();

// println!("{}", D.summarize());

  

let a = Pair { x: 1, y: 1 };

a.cmp_display();

  

// 给泛型添加方法

let circle = Circle { radius: 6 };

println!("{}", circle.to_string());

}

  

struct Pair<T> {

x: T,

y: T,

}

  

impl<T> Pair<T> {

fn new(x: T, y: T) -> Self {

Self { x, y }

}

}

  

impl<T: Display + PartialOrd> Pair<T> {

fn cmp_display(&self) {

if self.x >= self.y {

println!("The largest member is x = {}", self.x);

} else {

println!("The largest member is y = {}", self.y);

}

}

}

  

// 给泛型实现方法

use std::string::ToString;

  

struct Circle {

radius: i32,

}

  

impl ToString for Circle {

fn to_string(&self) -> String {

format!("Circle of radius {:?}", self.radius)

}

}
```
类似typescript的编程，但是更难用。

##### 生命周期
###### 避免悬垂引用
生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。
```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

```

###### 借用检查器
Rust 编译器有一个 借用检查器（borrow checker），它比较作用域来确保所有的借用都是有效的。
```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

```

###### 生命周期注解
```rust
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```
为了在函数签名中使用生命周期注解，需要在函数名和参数列表间的尖括号中声明泛型生命周期（lifetime）参数，就像泛型类型（type）参数一样。

我们希望函数签名表达如下限制：也就是这两个参数和返回的引用存活的一样久。（两个）参数和返回的引用的生命周期是相关的。

<div style="color: red">强行将参数中的生命周期，作用域拉到一致。避免引用失败。</div>
```rust
pub fn timer() {

let x = 5; // ----------+-- 'b

// |

let r = &x; // --+-- 'a |

// | |

// println!("r: {}", r); // | |

// --+ |

// ----------+

  

// !函数生命周期

let string1 = String::from("abcd");

// let string2 = "xyz";

// let result = longest(string1.as_str(), string2);

// println!("The longest string is {}", result);

{

let string2 = String::from("xyz");

let result = longest(string1.as_str(), string2.as_str());

// println!("The longest string is {}", result);

}

  

// !结构体声明周期

let novel = String::from("Call me Ishmael. Some years ago...");

let first_sentence = novel.split('.').next().expect("Could not find a '.'");

let i = ImportantExcerpt {

part: first_sentence,

};

println!("{:?}", i);

}

  

// &i32 // 引用

// &'a i32 // 带有显式生命周期的引用

// &'a mut i32 // 带有显式生命周期的可变引用

  

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

if x.len() > y.len() {

x

} else {

y

}

}

  

// 编译失败是因为有定义域不同

// fn longest1(x: &str, y: &str) -> str {

// if x.len() > y.len() {

// x

// } else {

// y

// }

// }

#[derive(Debug)]

struct ImportantExcerpt<'a> {

part: &'a str,

// 首先，这里有一个方法 level。其唯一的参数是 self 的引用，而且返回值只是一个 i32，并不引用任何值：

impl<'a> ImportantExcerpt<'a> {

fn level(&self) -> i32 {

3

}

}

// 适用于第三条生命周期省略规则的例子

impl<'a> ImportantExcerpt<'a> {

fn announce_and_return_part(&self, announcement: &str) -> &str {

println!("Attention please: {}", announcement);

self.part

}

}



}
```

<div style="color: green">生命周期省略（Lifetime Elision）</div>
在编写了很多 Rust 代码后，Rust 团队发现在特定情况下 Rust 程序员们总是重复地编写一模一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。接着 Rust 团队就把这些模式编码进了 Rust 编译器中，如此借用检查器在这些情况下就能推断出生命周期而不再强制程序员显式的增加注解。

这里我们提到一些 Rust 的历史是因为更多的明确的模式被合并和添加到编译器中是完全可能的。未来只会需要更少的生命周期注解。

被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）。这并不是需要程序员遵守的规则；这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。

省略规则并不提供完整的推断：如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，它不会猜测剩余引用的生命周期应该是什么。编译器会在可以通过增加生命周期注解来解决错误问题的地方给出一个错误提示，而不是进行推断或猜测。
<div style="color: skyblue">
函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
而返回值的生命周期被称为 输出生命周期（output lifetimes）。
</div>


编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。

- 第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数就有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
- 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
- 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)(译者注：这里涉及 rust 的面向对象参见 17 章)，那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
```rust
假设我们自己就是编译器。并应用这些规则来计算示例 10-25 中 first_word 函数签名中的引用的生命周期。开始时签名中的引用并没有关联任何生命周期：

fn first_word(s: &str) -> &str {
接着编译器应用第一条规则，也就是每个引用参数都有其自己的生命周期。我们像往常一样称之为 'a，所以现在签名看起来像这样：

fn first_word<'a>(s: &'a str) -> &str {
对于第二条规则，因为这里正好只有一个输入生命周期参数所以是适用的。第二条规则表明输入参数的生命周期将被赋予输出生命周期参数，所以现在签名看起来像这样：

fn first_word<'a>(s: &'a str) -> &'a str {
现在这个函数签名中的所有引用都有了生命周期，如此编译器可以继续它的分析而无须程序员标记这个函数签名中的生命周期。

让我们再看看另一个例子，这次我们从示例 10-20 中没有生命周期参数的 longest 函数开始：

fn longest(x: &str, y: &str) -> &str {
再次假设我们自己就是编译器并应用第一条规则：每个引用参数都有其自己的生命周期。这次有两个参数，所以就有两个（不同的）生命周期：

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
再来应用第二条规则，因为函数存在多个输入生命周期，它并不适用于这种情况。再来看第三条规则，它同样也不适用，这是因为没有 self 参数。应用了三个规则之后编译器还没有计算出返回值类型的生命周期。这就是在编译示例 10-20 的代码时会出现错误的原因：编译器使用所有已知的生命周期省略规则，仍不能计算出签名中所有引用的生命周期。

因为第三条规则真正能够适用的就只有方法签名，现在就让我们看看那种情况中的生命周期，并看看为什么这条规则意味着我们经常不需要在方法签名中标注生命周期。
```
###### 静态生命周期
这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
```rust
let s: &'static str = "I have a static lifetime.";
```
这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面值都是 'static 的。

###### 泛型，接口生命周期
```rust
use std::fmt::Display;

  

fn longest_with_an_announcement<'a, T>(

x: &'a str,

y: &'a str,

ann: T,

) -> &'a str

where

T: Display,

{

println!("Announcement! {}", ann);

if x.len() > y.len() {

x

} else {

y

}

}
```


#### 自动化测试
Edsger W. Dijkstra 在其 1972 年的文章【谦卑的程序员】（“The Humble Programmer”）中说到 “软件测试是证明 bug 存在的有效方法，而证明其不存在时则显得令人绝望的不足。”（“Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.”）这并不意味着我们不该尽可能地测试软件！

程序的正确性意味着代码如我们期望的那样运行。Rust 是一个相当注重正确性的编程语言，不过正确性是一个难以证明的复杂主题。Rust 的类型系统在此问题上下了很大的功夫，不过类型系统不可能捕获所有问题。为此，Rust 包含了编写自动化软件测试的功能支持。

Rust 中的测试函数是用来验证非测试代码是否是按照期望的方式运行的。测试函数体通常执行如下三种操作：
- 设置任何所需的数据或状态
- 运行需要测试的代码
- 断言其结果是我们所期望的

##### 生成模块lib
```shell
$ cargo new adder --lib
$ cd adder
```
##### 调用测试用例
```shell
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

// 测试函数的名称
running 1 test
test tests::it_works ... ok
// 测试结果
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
##### 编写测试
```rust
/*

!带入测试。

*/

  

pub fn add(left: usize, right: usize) -> usize {

left + right

}

  

#[derive(Debug)]

struct Rectangle {

width: u32,

height: u32,

}

  

impl Rectangle {

fn can_hold(&self, other: &Rectangle) -> bool {

self.width > other.width && self.height > other.height

}

}

  

pub fn add_two(a: i32) -> i32 {

// a + 3

a + 2

}

// todo 自定义失败信息

pub fn greeting(name: &str) -> String {

format!("Hello {}!", name)

// String::format("11")

}

  

// 使用should_panic 检查 panic

pub struct Guess {

value: i32,

}

impl Guess {

pub fn new(value: i32) -> Guess {

if value < 1 || value > 100 {

panic!("Guess value must be between 1 and 100, got {}.", value);

}

  

Guess { value }

}

}

  

#[cfg(test)]

mod tests {

use super::*;

  

#[test]

fn it_works() {

let result = add(2, 2);

assert_eq!(result, 4);

}

  

// 捕捉异常的枚举结构体，

#[test]

fn it_works_enmus() -> Result<(), String> {

if 2 + 2 == 4 {

Ok(())

} else {

Err(String::from("two plus two does not equal four"))

}

}

  

#[test]

fn exploration() {

assert_eq!(2 + 2, 4);

}

  

// 这里则是一个失败的测试例子

// #[test]

// fn another() {

// panic!("Make this test fail");

// }

  

// 结构体测试

use super::*;

#[test]

fn larger_can_hold_smaller() {

let larger = Rectangle {

width: 8,

height: 7,

};

let smaller = Rectangle {

width: 5,

height: 1,

};

// 断言 真假 这里报错

assert!(larger.can_hold(&smaller));

}

  

// 再次测试

#[test]

fn smaller_cannot_hold_larger() {

let larger = Rectangle {

width: 8,

height: 7,

};

let smaller = Rectangle {

width: 5,

height: 1,

};

  

assert!(!smaller.can_hold(&larger));

}

#[test]

fn it_adds_two() {

// 断言相等

assert_eq!(4, add_two(2));

}

  

#[test]

fn greeting_contains_name() {

let result = greeting("Carol");

// 直接打印自定义错误信息

assert!(result.contains("Carol"), "我是错误信息{}", result);

}

  

#[test]

#[should_panic(expected = "你不能传入正常值")]

fn greater_than_100() {

// 检测panic是否生效

Guess::new(2);

}

}
```
##### 控制测试
就像 cargo run 会编译代码并运行生成的二进制文件一样，cargo test 在测试模式下编译代码并运行生成的测试二进制文件。cargo test 产生的二进制文件的默认行为是并发运行所有的测试，并截获测试运行过程中产生的输出，阻止他们被显示出来，使得阅读测试结果相关的内容变得更容易。不过可以指定命令行参数来改变 cargo test 的默认行为。

可以将一部分命令行参数传递给 cargo test，而将另外一部分传递给生成的测试二进制文件。为了分隔这两种参数，需要先列出传递给 cargo test 的参数，接着是分隔符 --，再之后是传递给测试二进制文件的参数。运行 cargo test --help 会提示 cargo test 的有关参数，而运行 cargo test -- --help 可以提示在分隔符之后使用的有关参数。

###### 设置测试的线程
```rust
$ cargo test -- --test-threads=1
```
###### 显示成功的输出
```rust
$ cargo test -- --show-output
```
###### 单独测试 / 批量测试
只要名字是测试用例中函数的一部分，那符合的函数都会被使用
```rust
$ cargo test one_hundred
```
###### 忽略测试
需要加入#[ignore]
```rust
  
#[test]

#[ignore]

#[should_panic(expected = "你不能传入正常值")]

fn greater_than_100() {

// 检测panic是否生效

Guess::new(12);

}
```
```rust
$ cargo test -- --ignored
$ cargo test -- --include-ignored
```

##### 测试框架
单元测试（unit tests）与 集成测试（integration tests）。单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。
###### 单元测试
将测试用例写入到模块本身中
```rust
/*

!带入测试。

*/

  

pub fn add(left: usize, right: usize) -> usize {

left + right

}

  

#[derive(Debug)]

struct Rectangle {

width: u32,

height: u32,

}

  

impl Rectangle {

fn can_hold(&self, other: &Rectangle) -> bool {

self.width > other.width && self.height > other.height

}

}

  

pub fn add_two(a: i32) -> i32 {

// a + 3

a + 2

}

// todo 自定义失败信息

pub fn greeting(name: &str) -> String {

format!("Hello {}!", name)

// String::format("11")

}

  

// 使用should_panic 检查 panic

pub struct Guess {

value: i32,

}

impl Guess {

pub fn new(value: i32) -> Guess {

if value < 1 || value > 100 {

panic!("Guess value must be between 1 and 100, got {}.", value);

}

  

Guess { value }

}

}

  

#[cfg(test)]

mod tests {

use super::*;

  

#[test]

fn it_works() {

let result = add(2, 2);

assert_eq!(result, 4);

}

  

// 捕捉异常的枚举结构体，

#[test]

fn it_works_enmus() -> Result<(), String> {

if 2 + 2 == 4 {

Ok(())

} else {

Err(String::from("two plus two does not equal four"))

}

}

  

#[test]

fn exploration() {

assert_eq!(2 + 2, 4);

}

  

// 这里则是一个失败的测试例子

// #[test]

// fn another() {

// panic!("Make this test fail");

// }

  

// 结构体测试

use super::*;

#[test]

fn larger_can_hold_smaller() {

let larger = Rectangle {

width: 8,

height: 7,

};

let smaller = Rectangle {

width: 5,

height: 1,

};

// 断言 真假 这里报错

assert!(larger.can_hold(&smaller));

}

  

// 再次测试

#[test]

fn smaller_cannot_hold_larger() {

let larger = Rectangle {

width: 8,

height: 7,

};

let smaller = Rectangle {

width: 5,

height: 1,

};

  

assert!(!smaller.can_hold(&larger));

}

#[test]

fn it_adds_two() {

// 断言相等

assert_eq!(4, add_two(2));

}

  

#[test]

fn greeting_contains_name() {

let result = greeting("Carol");

// 直接打印自定义错误信息

assert!(result.contains("Carol"), "我是错误信息{}", result);

}

  

#[test]

#[ignore]

#[should_panic(expected = "你不能传入正常值")]

fn greater_than_100() {

// 检测panic是否生效

Guess::new(12);

}

}
```
###### 集成测试
在 Rust 中，集成测试对于你需要测试的库来说完全是外部的。同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API。集成测试的目的是测试库的多个部分能否一起正常工作。一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。为了创建集成测试，你需要先创建一个 tests 目录。
```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

```
<p style='color: red;'>如果src的存在同级tests，则只会使用tests中的rs文件。
</p>
也可以选择用哪个测试用例。
```shell
$ cargo test --test integration_test
```
如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

这就是许多 Rust 二进制项目使用一个简单的 src/main.rs 调用 src/lib.rs 中的逻辑的原因之一。因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。

![](readme.assets/Pasted%20image%2020230806184721.png)

#### 构建命令行程序
Rust 的运行速度、安全性、单二进制文件输出和跨平台支持使其成为创建命令行程序的绝佳选择，所以我们的项目将创建一个我们自己版本的经典命令行搜索工具：grep。grep 是 “Globally search a Regular Expression and Print.” 的首字母缩写。grep 最简单的使用场景是在特定文件中搜索指定字符串。为此，grep 获取一个文件路径和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

在这个过程中，我们会展示如何让我们的命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。打印到标准错误控制流（stderr）而不是标准输出（stdout），例如这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。
```shell
# 创建新项目
$ cargo new minigrep
```
执行命令，检索文件
```shell
cargo run -- searchstring example-filename.txt
```

##### 读写参数
```rust
// 调用函数，使用外部传参

use std::env;

  

fn main() {

// 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）

let args: Vec<String> = env::args().collect();

// 打印到异常控制台

// dbg!(args);

/*

[src/main.rs:7] args = [

"target/debug/minigrep", // * 这是二进制文件名称

"searchstring", // * 第一个参数

"example-filename.txt", // * 第二参数

]˝

*/

  

// 获取参数

let pathName = &args[0];

let query = &args[1];

let file_path = &args[2];

println!("zero{}", pathName);

println!("one{}", query);

println!("two{}", file_path);

}
```

##### 读写文件
```rust
// 调用函数，使用外部传参

use std::env;

use std::fs;

  

/*

main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

  

将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。

当命令行解析逻辑比较小时，可以保留在 main.rs 中。

当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：

  

使用参数值调用命令行解析逻辑

设置任何其他的配置

调用 lib.rs 中的 run 函数

如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

*/

  

fn main() {

// 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）

let args: Vec<String> = env::args().collect();

// 打印到异常控制台

// dbg!(args);

/*

[src/main.rs:7] args = [

"target/debug/minigrep", // * 这是二进制文件名称

"searchstring", // * 第一个参数

"example-filename.txt", // * 第二参数

]˝

*/

  

// 获取参数

// let pathName = &args[0];

let query = &args[1];

let file_path = &args[2];

// println!("zero{}", pathName);

// println!("one{}", query);

// println!("two{}", file_path);

  

// 读写文件

println!("In file {}", file_path);

let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

println!("With text:\n{contents}");

  

}
```
##### 标准写法
```rust
// 调用函数，使用外部传参

use std::env;

use std::error::Error;

use std::fs;

use std::process;

  

/*

main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

  

将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。

当命令行解析逻辑比较小时，可以保留在 main.rs 中。

当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：

  

使用参数值调用命令行解析逻辑

设置任何其他的配置

调用 lib.rs 中的 run 函数

如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

*/

  

fn main() {

// 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）

// let args: Vec<String> = env::args().collect();

// 打印到异常控制台

// dbg!(args);

/*

[src/main.rs:7] args = [

"target/debug/minigrep", // * 这是二进制文件名称

"searchstring", // * 第一个参数

"example-filename.txt", // * 第二参数

]˝

*/

  

// 获取参数

// let pathName = &args[0];

// let query = &args[1];

// let file_path = &args[2];

// println!("zero{}", pathName);

// println!("one{}", query);

// println!("two{}", file_path);

  

// 读写文件

// println!("In file {}", file_path);

// let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

// println!("With text:\n{contents}");

  

let args: Vec<String> = env::args().collect();

let config = Config::build(&args).unwrap_or_else(|err| {

println!("Problem parsing arguments: {err}");

process::exit(1);

});

  

// let contents =

// fs::read_to_string(config.file_path).expect("Should have been able to read the file");

  

if let Err(e) = run(config) {

println!("Application error: {e}");

process::exit(1);

}

}

  

struct Config {

query: String,

file_path: String,

}

  

// fn parse_config(args: &[String]) -> Config {

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Config { query, file_path }

// }

  

impl Config {

fn build(args: &[String]) -> Result<Config, &'static str> {

if args.len() < 3 {

return Err("not enough arguments");

}

  

let query = args[1].clone();

let file_path = args[2].clone();

  

Ok(Config { query, file_path })

}

}

  

fn run(config: Config) -> Result<(), Box<dyn Error>> {

let contents = fs::read_to_string(config.file_path)?;

println!("With text:\n{contents}");

Ok(())

}
```

##### 模块化
main
```rust
// 调用函数，使用外部传参

use std::env;

// use std::error::Error;

// use std::fs;

use minigrep::Config;

use std::process;

  

/*

main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

  

将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。

当命令行解析逻辑比较小时，可以保留在 main.rs 中。

当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：

  

使用参数值调用命令行解析逻辑

设置任何其他的配置

调用 lib.rs 中的 run 函数

如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

*/

  

fn main() {

// 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）

// let args: Vec<String> = env::args().collect();

// 打印到异常控制台

// dbg!(args);

/*

[src/main.rs:7] args = [

"target/debug/minigrep", // * 这是二进制文件名称

"searchstring", // * 第一个参数

"example-filename.txt", // * 第二参数

]˝

*/

  

// 获取参数

// let pathName = &args[0];

// let query = &args[1];

// let file_path = &args[2];

// println!("zero{}", pathName);

// println!("one{}", query);

// println!("two{}", file_path);

  

// 读写文件

// println!("In file {}", file_path);

// let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

// println!("With text:\n{contents}");

  

let args: Vec<String> = env::args().collect();

let config = Config::build(&args).unwrap_or_else(|err| {

println!("Problem parsing arguments: {err}");

process::exit(1);

});

  

// let contents =

// fs::read_to_string(config.file_path).expect("Should have been able to read the file");

  

// if let Err(e) = run(config) {

// println!("Application error: {e}");

// process::exit(1);

// }

  

if let Err(e) = minigrep::run(config) {

println!("Application error: {e}");

process::exit(1);

}

}

  

// struct Config {

// query: String,

// file_path: String,

// }

  

// fn parse_config(args: &[String]) -> Config {

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Config { query, file_path }

// }

  

// impl Config {

// fn build(args: &[String]) -> Result<Config, &'static str> {

// if args.len() < 3 {

// return Err("not enough arguments");

// }

  

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Ok(Config { query, file_path })

// }

// }

  

// fn run(config: Config) -> Result<(), Box<dyn Error>> {

// let contents = fs::read_to_string(config.file_path)?;

// println!("With text:\n{contents}");

// Ok(())

// }
```
##### 处理环境变量
lib.rs
```rust
use std::error::Error;

use std::fs;

use std::env;

  

// pub struct Config {

// pub query: String,

// pub file_path: String,

// }

  

pub struct Config {

pub query: String,

pub file_path: String,

pub ignore_case: bool,

}

  

// impl Config {

// pub fn build(args: &[String]) -> Result<Config, &'static str> {

// if args.len() < 3 {

// return Err("not enough arguments");

// }

  

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Ok(Config { query, file_path })

// }

// }

  

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

// let contents = fs::read_to_string(config.file_path)?;

// // println!("With text:\n{contents}");

// for line in search(&config.query, &contents) {

// // 找到我们想要的那行

// println!("{line}");

// }

// Ok(())

// }

  

impl Config {

pub fn build(args: &[String]) -> Result<Config, &'static str> {

if args.len() < 3 {

return Err("not enough arguments");

}

  

let query = args[1].clone();

let file_path = args[2].clone();

// IGNORE_CASE=1 cargo run to poem.txt 设置环境变量执行语句

let ignore_case = env::var("IGNORE_CASE").is_ok();

  

Ok(Config {

query,

file_path,

ignore_case,

})

}

}

  

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

let contents = fs::read_to_string(config.file_path)?;

  

let results = if config.ignore_case {

search_case_insensitive(&config.query, &contents)

} else {

search(&config.query, &contents)

};

  

for line in results {

println!("{line}");

}

  

Ok(())

}

  

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

let mut results = Vec::new();

  

for line in contents.lines() {

if line.contains(query) {

// 第二行的productive刚好满足条件

results.push(line);

}

}

  

results

}

  

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

let query = query.to_lowercase();

let mut results = Vec::new();

  

for line in contents.lines() {

if line.to_lowercase().contains(&query) {

results.push(line);

}

}

  

results

}

  

#[cfg(test)]

mod tests {

use super::*;

  

#[test]

fn one_result() {

let query = "duct";

let contents = "\

Rust:

safe, fast, productive.

Pick three.";

  

// 测试是否相等

assert_eq!(vec!["safe, fast, productive."], search(query, contents));

}

  

// 大小写不敏感

#[test]

fn case_sensitive() {

let query = "duct";

let contents = "\

Rust:

safe, fast, productive.

Pick three.

Duct tape.";

  

assert_eq!(vec!["safe, fast, productive."], search(query, contents));

}

  

#[test]

fn case_insensitive() {

let query = "rUsT";

let contents = "\

Rust:

safe, fast, productive.

Pick three.

Trust me.";

  

assert_eq!(

vec!["Rust:", "Trust me."],

search_case_insensitive(query, contents)

);

}

}
```
##### 使用错误输出
main.rs
```rust
// 调用函数，使用外部传参

use std::env;

// use std::error::Error;

// use std::fs;

use minigrep::Config;

use std::process;

  

/*

main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导。这些过程有如下步骤：

  

将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。

当命令行解析逻辑比较小时，可以保留在 main.rs 中。

当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

经过这些过程之后保留在 main 函数中的责任应该被限制为：

  

使用参数值调用命令行解析逻辑

设置任何其他的配置

调用 lib.rs 中的 run 函数

如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

*/

  

fn main() {

// 需要一个 Rust 标准库提供的函数 std::env::args。这个函数返回一个传递给程序的命令行参数的 迭代器（iterator）

// let args: Vec<String> = env::args().collect();

// 打印到异常控制台

// dbg!(args);

/*

[src/main.rs:7] args = [

"target/debug/minigrep", // * 这是二进制文件名称

"searchstring", // * 第一个参数

"example-filename.txt", // * 第二参数

]˝

*/

  

// 获取参数

// let pathName = &args[0];

// let query = &args[1];

// let file_path = &args[2];

// println!("zero{}", pathName);

// println!("one{}", query);

// println!("two{}", file_path);

  

// 读写文件

// println!("In file {}", file_path);

// let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

// println!("With text:\n{contents}");

  

let args: Vec<String> = env::args().collect();

let config = Config::build(&args).unwrap_or_else(|err| {

// println!("Problem parsing arguments: {err}");

// ! 使用eprintln! 可以让错误信息不被输出

eprintln!("Problem parsing arguments: {err}");

process::exit(1);

});

  

// let contents =

// fs::read_to_string(config.file_path).expect("Should have been able to read the file");

  

// if let Err(e) = run(config) {

// println!("Application error: {e}");

// process::exit(1);

// }

  

if let Err(e) = minigrep::run(config) {

// println!("Application error: {e}");

eprintln!("Application error: {e}");

process::exit(1);

}

}

  

// struct Config {

// query: String,

// file_path: String,

// }

  

// fn parse_config(args: &[String]) -> Config {

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Config { query, file_path }

// }

  

// impl Config {

// fn build(args: &[String]) -> Result<Config, &'static str> {

// if args.len() < 3 {

// return Err("not enough arguments");

// }

  

// let query = args[1].clone();

// let file_path = args[2].clone();

  

// Ok(Config { query, file_path })

// }

// }

  

// fn run(config: Config) -> Result<(), Box<dyn Error>> {

// let contents = fs::read_to_string(config.file_path)?;

// println!("With text:\n{contents}");

// Ok(())

// }
```
####  迭代器和闭包
programming）。函数式编程风格通常包含将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。

本章我们不会讨论函数式编程是或不是什么的问题，而是展示 Rust 的一些在功能上与其他被认为是函数式语言类似的特性。

更具体的，我们将要涉及：

闭包（Closures），一个可以储存在变量里的类似函数的结构
迭代器（Iterators），一种处理元素序列的方式
如何使用闭包和迭代器来改进第十二章的 I/O 项目。
闭包和迭代器的性能。（剧透警告： 他们的速度超乎你的想象！）


##### 闭包
Rust 的 闭包（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获被定义时所在作用域中的值。


```rust

```











#### ⭐️智能指针


#### ⭐️安全并发


#### 面向对象编程


#### 模式与模式匹配


#### 高级特征


#### ⭐️多线程与多进程



#### 标准库


#### Cargo用法
- 查看cargo版本
```shell
$ cargo --version
```
- 创建新项目
```shell
$ cargo new hello_cargo
```
- 构建
```shell
$ cargo build
```
- 代码检查
```
$ cargo check
```
- 添加依赖
```
$ cargo add rand
```
- 升级包
	- 默认将依赖库版本提升到稳定版本，而不是最新版本。
	- 如果你想让依赖提升到最新版本，还是需要手动改写cargo.toml
```
cargo update
```
- 查看依赖，运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开。依赖文档竟然是
```shell
$ cargo doc --open
```




##### 打包发布




#### 嵌入式开发

```markdown
# 裸机开发

裸机开发是指在没有操作系统或运行时环境的支持下，直接在硬件上运行程序的一种开发方式。这通常用于嵌入式系统或一些特殊的环境，要求程序员直接控制底层硬件资源。以下是裸机开发的一般原理：

1. **启动代码（Bootstrapping）：** 在裸机开发中，首先需要一个启动代码，该代码负责初始化硬件环境、设置堆栈、加载程序等。这通常是由硬件提供的引导程序（bootloader）或者自定义的启动代码。
    
2. **硬件初始化：** 在启动代码中，需要进行硬件的初始化工作，包括但不限于处理器初始化、内存控制器配置、外设初始化等。这一步确保硬件处于一个稳定的状态，可以正常执行程序。
    
3. **编写底层代码：** 在裸机开发中，程序员需要直接编写与目标硬件相关的底层代码，包括访问寄存器、配置中断、操作外设等。这通常需要参考硬件手册和相关文档。
    
4. **中断处理：** 裸机系统通常需要处理中断，包括时钟中断、外设中断等。程序员需要编写中断服务程序（Interrupt Service Routines，ISR）来响应这些中断事件。
    
5. **无操作系统环境：** 在裸机开发中，没有操作系统提供的系统调用、内存管理和任务调度等功能。程序员需要自己管理内存、处理任务调度、进行输入输出等。
    
6. **链接脚本：** 为了将程序正确地加载到内存中，需要编写链接脚本，定义程序的内存布局、代码段、数据段等信息。
    
7. **调试：** 由于裸机环境没有标准的调试工具，调试通常通过串口输出、LED指示等方式进行。也可以使用仿真器或调试器来辅助。
    
8. **交叉编译：** 由于裸机开发通常涉及到特定硬件架构，需要使用交叉编译工具链，将程序编译为目标硬件能够执行的二进制文件。
    

总体而言，裸机开发要求程序员对底层硬件有深入的了解，能够直接控制硬件资源。这种开发方式适用于嵌入式系统、实时系统等场景，对性能和资源利用的要求较高。
```

##### 开发板




##### 系统构建

#### Shell开发


#### webAssembly开发
Jco 1.0 发布
我们很高兴地宣布 Jco 1.0 版本：为 WebAssembly 组件和 WASI 0.2 1 构建的原生 Javascript WebAssembly 工具链和运行时。Jco 可以在 Node.js 内原生运行 Wasm 组件，从而可以轻松获取用不同编程编写的库语言并使用 Node.js 运行时执行它们。通过实现整个 WASI 0.2 API 接口，这些组件可以访问网络、文件系统以及 Node.js 运行时中可用的其他系统 API。



##### 例子
https://mp.weixin.qq.com/s/z8xYSdSYnhJ3LN74kxQo2A
```rust
// 1.安装 GreptimeDB 和 WasmEdge
// 安装 GreptimeDB 和 WasmEdge

use mysql_async::{
    prelude::*, Opts, OptsBuilder, Pool, PoolConstraints, PoolOpts, Result, SslOpts,
};
use time::PrimitiveDateTime;

fn get_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
        if opts
            .db_name()
            .expect("a database name is required")
            .is_empty()
        {
            panic!("database name is empty");
        }
        url
    } else {
        "mysql://root:pass@127.0.0.1:3306/mysql".into()
    }
}

#[derive(Debug)]
struct CpuMetric {
    hostname: String,
    environment: String,
    usage_user: f64,
    usage_system: f64,
    usage_idle: f64,
    ts: i64,
}

impl CpuMetric {
    fn new(
        hostname: String,
        environment: String,
        usage_user: f64,
        usage_system: f64,
        usage_idle: f64,
        ts: i64,
    ) -> Self {
        Self {
            hostname,
            environment,
            usage_user,
            usage_system,
            usage_idle,
            ts,
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Alternative: The "easy" way with a default connection pool
    // let pool = Pool::new(Opts::from_url(&*get_url()).unwrap());
    // let mut conn = pool.get_conn().await.unwrap();

    // Below we create a customized connection pool
    let opts = Opts::from_url(&*get_url()).unwrap();
    let mut builder = OptsBuilder::from_opts(opts);
    // Disabled after we figured out tls issue.
    if std::env::var("DATABASE_SSL").is_ok() {
        let ssl_opts = SslOpts::default().with_danger_accept_invalid_certs(true);
        // .with_danger_skip_domain_validation(true);
        builder = builder.ssl_opts(ssl_opts);
    }
    // The connection pool will have a min of 5 and max of 10 connections.
    let constraints = PoolConstraints::new(1, 2).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);

    let pool = Pool::new(builder.pool_opts(pool_opts));
    let mut conn = pool.get_conn().await.unwrap();
    // Create table if not exists
    r"CREATE TABLE IF NOT EXISTS wasmedge_example_cpu_metrics (
    hostname STRING,
    environment STRING,
    usage_user DOUBLE,
    usage_system DOUBLE,
    usage_idle DOUBLE,
    ts TIMESTAMP,
    TIME INDEX(ts),
    PRIMARY KEY(hostname, environment)
);"
    .ignore(&mut conn)
    .await?;
    println!("Table created!");

    println!("Ingest some data...");
    let metrics = vec![
        CpuMetric::new(
            "host0".into(),
            "test".into(),
            32f64,
            3f64,
            4f64,
            1680307200050,
        ),
        CpuMetric::new(
            "host1".into(),
            "test".into(),
            29f64,
            32f64,
            50f64,
            1680307200050,
        ),
        CpuMetric::new(
            "host0".into(),
            "test".into(),
            32f64,
            3f64,
            4f64,
            1680307260050,
        ),
        CpuMetric::new(
            "host1".into(),
            "test".into(),
            29f64,
            32f64,
            50f64,
            1680307260050,
        ),
        CpuMetric::new(
            "host0".into(),
            "test".into(),
            32f64,
            3f64,
            4f64,
            1680307320050,
        ),
        CpuMetric::new(
            "host1".into(),
            "test".into(),
            29f64,
            32f64,
            50f64,
            1680307320050,
        ),
    ];

    r"INSERT INTO wasmedge_example_cpu_metrics (hostname, environment, usage_user, usage_system, usage_idle, ts)
      VALUES (:hostname, :environment, :usage_user, :usage_system, :usage_idle, :ts)"
        .with(metrics.iter().map(|metric| {
            params! {
                "hostname" => &metric.hostname,
                "environment" => &metric.environment,
                "usage_user" => metric.usage_user,
                "usage_system" => metric.usage_system,
                "usage_idle" => metric.usage_idle,
                "ts" => metric.ts,
            }
        }))
        .batch(&mut conn)
        .await?;

    // query data
    println!("Query some data");
    let loaded_metrics = "SELECT * FROM wasmedge_example_cpu_metrics"
        .with(())
        .map(
            &mut conn,
            |(hostname, environment, usage_user, usage_system, usage_idle, raw_ts): (
                String,
                String,
                f64,
                f64,
                f64,
                PrimitiveDateTime,
            )| {
                let ts = raw_ts.assume_utc().unix_timestamp() * 1000;
                CpuMetric::new(
                    hostname,
                    environment,
                    usage_user,
                    usage_system,
                    usage_idle,
                    ts,
                )
            },
        )
        .await?;
    println!("{:?}", loaded_metrics);

    // Dropped connection will go to the pool
    drop(conn);
    // The Pool must be disconnected explicitly because
    // it's an asynchronous operation.
    pool.disconnect().await?;
    Ok(())
}


```
随后编译它，再执行它
```shell
$ cargo buildls -lh target/wasm32-wasi/debug/greptimedb.wasm
$ wasmedge --env "DATABASE_URL=mysql://localhost:4002/public" target/wasm32-wasi/debug/greptimedb.wasm
```

# 深入理解 Linux & Rust

Linux的层级架构
每个操作系统都有一个内核，内核封装了底层硬件设备管理、内存管理、网络数据协议转化和收发传输、文件系统读写等。从这个图可以看到，内核将系统硬件与应用程序进程连接起来，隐藏了上层下层交互的一些细节，各司其职。
![](readme.assets/Pasted%20image%2020231102015536.png)

这些分层包括：

	用户空间程序
	编译器
	终端
	防火墙
	系统调用的跨平台API(特定于平台的系统调用包装API)
	Rust标准库
	libc（或等效的API）
	kernel，操作系统的核心模块
	系统资源
	内存
	文件系统
	网络
	硬件和其他设备（包括键盘、鼠标、监视器、磁盘驱动器）


## Rust的标准库的功能划分

而Rust标准库，很好的利用了操作系统内核提供的API。

Rust标准库是Rust程序进入Linux操作系统内核函数的主要接口，它在内部使用libc(在Windows系统使用其他等效的库）来调用内核提供的系统调用。

从Rust程序中发起系统调用，以实现管理和操作各种系统资源（如图）。

![](readme.assets/Pasted%20image%2020231102015652.png)

图片libc（或其变体）为类UNIX操作系统上的系统调用提供了一个包装器，如Linux内核实现了POSIX标准指定的数百个POSIX API（对于Windows，系统调用有等效的API，也实现了POSIX标准[1]）。

作为标准库，Rust标准库是跨平台的，Rust标准库的系统调用的细节是从Rust开发人员那里抽象出来的。Rust也支持不依赖于标准库的运行方式（no_std 方式），Rust直接操控底层硬件（如应用在嵌入式系统开发场景），此时Rust就做了操作系统本身的工作。

对于大部分软件开发工程师而言，他们用Rust主要开发应用层软件，也就是运行在用户空间的程序。它们基于标准库编写，实现各种业务功能。应用层的软件并非所有模块和函数都涉及到系统调用（例如一些用于操作字符串和处理错误的函数，就无需调用系统调用）。

## Rust标准库四大类：

### 第一类，Rust语言原语

即Rust Language Primitives：Rust 语言的基本元素或基本类型（如下图）。

如有符号整数、布尔值、浮点数、字符、字符串、数组、元组、切片。这些由Rust编译器负责实现。

![](readme.assets/Pasted%20image%2020231102015907.png)

Rust标准包括原语，并在它们之上构建。

### 第二类，alloc crate

与堆分配值的内存分配相关的类型、函数和特征。
```
包括集合（Vec、String等集合）、智能指针类型（Box<T>）、引用计数指针（Rc<T>）和原子引用计数指针（Arc<T>））。
```

### 第三类，core crate

作为Rust标准库的基础。充当Rust语言与标准库之间的链接，提供在Rust原语之上实现的类型、特征、常量和函数，并为所有Rust代码提供基础构建块，它是跨平台的，没有任何指向操作系统或其他外部依赖的链接。由于较少直接用到core crate，所以本文不做过多介绍。


### 第四类，模块（标准库的其他crate）

是标准库的一部分，模块crate包括针对并发、I/O，文件系统、网络、异步I/O、错误处理等功能，以及与特定操作系统相关的函数，Rust的官网对std有专门的文档[2]。例如

为用户程序在多个线程上并发运行的功能在std::thread模块中；
用于处理同步I/O的功能在std::io模块中提供；
针对特定os的模块，主要在std::os模块中实现。
下图展示了Rust标准库各个领域功能涉及到的具体std模块（如std::io、std::os等）
![](readme.assets/Pasted%20image%2020231102020142.png)

![](readme.assets/Pasted%20image%2020231102020213.png)
![](readme.assets/Pasted%20image%2020231102020223.png)

![](readme.assets/Pasted%20image%2020231102020231.png)
![](readme.assets/Pasted%20image%2020231102020242.png)
![](readme.assets/Pasted%20image%2020231102020253.png)


## 字符串格式化

| 占位符              | 含义                   | 示例代码                            | 输出示例         |
| ---------------- | -------------------- | ------------------------------- | ------------ |
| `{}`             | 默认格式（Display）        | `println!("{}", 42)`            | `42`         |
| `{:?}`           | 调试格式（Debug）          | `println!("{:?}", [1, 2])`      | `[1, 2]`     |
| `{:#?}`          | 美化调试格式（Pretty Debug） | `println!("{:#?}", vec![1, 2])` | 多行缩进格式       |
| `{{` / `}}`      | 输出大括号 `{` 或 `}`      | `println!("{{}}")`              | `{}`         |
| `{:width$}`      | 固定宽度，右对齐（默认）         | `println!("{:5}", "hi")`        | `' hi'`      |
| `{:<width$}`     | 左对齐，占 `width` 宽度     | `println!("{:<5}", "hi")`       | `'hi '`      |
| `{:>width$}`     | 右对齐，占 `width` 宽度     | `println!("{:>5}", "hi")`       | `' hi'`      |
| `{:^width$}`     | 居中对齐                 | `println!("{:^5}", "hi")`       | `' hi '`     |
| `{:*>width$}`    | 使用 `*` 填充，右对齐        | `println!("{:*>5}", "hi")`      | `'***hi'`    |
| `{:_<width$}`    | 使用 `_` 填充，左对齐        | `println!("{:_<5}", "hi")`      | `'hi___'`    |
| `{:b}`           | 二进制格式                | `println!("{:b}", 5)`           | `101`        |
| `{:o}`           | 八进制格式                | `println!("{:o}", 9)`           | `11`         |
| `{:x}`           | 十六进制（小写）             | `println!("{:x}", 255)`         | `ff`         |
| `{:X}`           | 十六进制（大写）             | `println!("{:X}", 255)`         | `FF`         |
| `{:#b}`          | 带前缀的二进制（`0b`）        | `println!("{:#b}", 5)`          | `0b101`      |
| `{:#x}`          | 带前缀的十六进制（`0x`）       | `println!("{:#x}", 255)`        | `0xff`       |
| `{:.N}`          | 浮点保留小数点后 N 位         | `println!("{:.2}", 3.14159)`    | `3.14`       |
| `{:<width.prec}` | 浮点左对齐                | `println!("{:<8.2}", 3.14)`     | `'3.14 '`    |
| `{:>width.prec}` | 浮点右对齐                | `println!("{:>8.2}", 3.14)`     | `' 3.14'`    |
| `{:e}`           | 科学计数法（小写 e）          | `println!("{:e}", 1234.567)`    | `1.234567e3` |
| `{:E}`           | 科学计数法（大写 E）          | `println!("{:E}", 1234.567)`    | `1.234567E3` |
| `{:0width$}`     | 宽度不足前导补零             | `println!("{:05}", 42)`         | `00042`      |
| `{:02}`          | 固定两位宽度，不足补零          | `println!("{:02}", 4)`          | `04`         |

