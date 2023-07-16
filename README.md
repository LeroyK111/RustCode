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





#### 结构体


#### 枚举和模式


#### 包，库，creat模块管理


#### 常见集合


#### ⭐️错误处理


#### 泛型、Trait、生命周期


#### 自动化测试


#### 构建命令行程序


####  迭代器和闭包

#### ⭐️智能指针


#### 安全并发


#### 面向对象编程


#### 模式与模式匹配


#### 高级特征


#### 多线程与多进程



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


