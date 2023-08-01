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

// let user2 = User {

// active: user1.active,

// username: user1.username,

// email: String::from("another@example.com"),

// sign_in_count: user1.sign_in_count,

// };

// 使用语法糖，进行解构赋值

let user3 = User {

email: String::from("another@example.com"),

..user1

};

// user1被清理掉了，user3能正常使用。

// println!("{}", user1.email)

// println!("{}", user2.email)

// println!("{}", user3.sign_in_count);

// clone 可以进行属性的深复制, 但结构体不能深复制

let user4 = user3.email.clone();

// println!("{}", user4)

  

// 元组结构体

let black = Color(0, 0, 0);

  

// 类单元结构体

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

// let width1 = 30;

// let height1 = 50;

// println!("{}", area(width1, height1));

  

// let rect1 = (30, 50);

// println!("{}", area1(rect1));

  

let rect2 = Rectangle {

width: 30,

height: 50,

};

println!("{}", area2(&rect2));

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

  

// 改写结构体，这里很像js的prototype，高级对象方法。

impl Rectangle {

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

println!("The rectangle has a nonzero width; it is {}", rect1.width);

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



#### 泛型、Trait、生命周期


#### 自动化测试


#### 构建命令行程序


####  迭代器和闭包

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
##### 开发板

##### 系统构建

#### Shell开发


#### webAssembly开发


