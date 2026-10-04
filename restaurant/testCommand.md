
## 1. 先理解分隔符 --

命令的结构是：

```text
cargo test [Cargo 的参数和测试过滤词] -- [测试执行程序的参数]
```

例如：

```powershell
cargo test add -- --show-output --test-threads=1
```

读成：

```text
cargo test          编译并运行测试
add                 筛选名称包含 add 的测试
--                  后面的参数交给测试执行程序
--show-output       也显示成功测试的日志
--test-threads=1    一次只运行一个测试

--lib 只测试库目标，通常以 src/lib.rs 为入口, 不加则是 main 目标
--nocapture 不捕获测试输出，让日志直接显示
```

`--` 是参数分隔符，不是 Rust 语法。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

查帮助时也分两种：

```powershell
cargo test --help
cargo test -- --help
```

前者查看 Cargo 的选项，后者查看测试执行程序的选项。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

## 2. 按名称选择测试

假设你有三个测试：

```rust
#[test]
fn add_positive() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn add_negative() {
    assert_eq!(-2 + -3, -5);
}

#[test]
fn divide_success() {
    assert_eq!(10 / 2, 5);
}
```

对应命令：

| 命令 | 运行哪些测试 |
|---|---|
| `cargo test` | 三个都运行 |
| `cargo test add` | 两个名称包含 `add` 的测试 |
| `cargo test add_positive` | 名称包含 `add_positive` 的测试 |
| `cargo test divide` | 名称包含 `divide` 的测试 |

默认是**子字符串匹配**，不是必须完整相等。模块路径也是完整测试名称的一部分，所以可以按模块名筛选。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

需要严格匹配某个测试时，可以写：

```powershell
cargo test tests::add_positive -- --exact
```

这里假设它位于 `mod tests` 中；实际路径以你的测试列表为准。查看名称可以运行： [doc.rust-lang](https://doc.rust-lang.org/rustc/tests/index.html)

```powershell
cargo test -- --list
```

记忆：**命令后面的名字是过滤词，不是直接调用函数。**

## 3. 并行和日志怎么控制？

### 默认并行

测试可能同时运行，因此不应相互依赖，也不应随意修改同一份共享状态。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

结合你之前的文件练习：

```text
测试 A：往 hello.txt 写入 "Alice"
测试 B：往 hello.txt 写入 "Bob"
测试 A：读取文件，却读到了 "Bob"
```

失败原因可能是测试互相干扰，而不是读取函数有 bug。

可以临时改为串行：

```powershell
cargo test -- --test-threads=1
```

但它只是避免测试同时执行，不能自动清理前一个测试留下的文件或状态。更稳妥的做法是每个测试使用独立数据、独立文件，并自行清理。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

### 日志默认被捕获

```rust
#[test]
fn example() {
    println!("开始测试");
    assert_eq!(2 + 3, 5);
}
```

默认运行时，测试通过，你可能只看到：

```text
test tests::example ... ok
```

这不代表 `println!` 没执行，而是输出被捕获了；失败测试的输出通常会显示。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

两种显示方式：

| 参数 | 日志如何显示 |
|---|---|
| `--show-output` | 全部测试结束后，按测试分组显示，包括成功测试 |
| `--no-capture` | 测试运行时直接显示，多个并行测试的日志可能交错 |

这两者的区别是“结束后整理显示”与“运行时直接显示”。 [doc.rust-lang](https://doc.rust-lang.org/rustc/tests/index.html)

对你当前边学边观察日志的习惯，可以使用：

```powershell
cargo test -- --show-output --test-threads=1
```

需要实时看打印时，换成：

```powershell
cargo test -- --no-capture --test-threads=1
```

## 4. 忽略耗时测试

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 耗时操作
}
```

`#[ignore]` 的意思是“默认不执行”，不是“删除测试”或“不编译”。所以其中有编译错误，仍然会影响测试构建。 [doc.rust-lang](https://doc.rust-lang.org/rustc/tests/index.html)

| 命令 | 执行范围 |
|---|---|
| `cargo test` | 未被忽略的测试 |
| `cargo test -- --ignored` | 只执行被忽略的测试 |
| `cargo test -- --include-ignored` | 普通测试和被忽略的测试都执行 |
| `cargo test expensive_test -- --ignored` | 执行匹配名称的被忽略测试 |

仅用名称筛选不会自动解除 `#[ignore]`，需要配合相应选项。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

这适合耗时测试或需要特殊环境的测试，不应拿来长期隐藏本来应该修复的失败。

## 5. 看懂结果和记忆

例如：

```text
2 passed; 1 failed; 1 ignored; 3 filtered out
```

| 字段 | 含义 |
|---|---|
| `passed` | 执行并通过 |
| `failed` | 执行但失败 |
| `ignored` | 因忽略标记而未执行 |
| `filtered out` | 没匹配筛选条件，未执行 |

注意，“未执行”和“通过”不是一回事。 [kaisery.github](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

你现在最常用的三条命令就够了：

```powershell
# 平时检查
cargo test

# 只检查某类功能
cargo test add

# 学习调试：显示日志，一次执行一个
cargo test -- --show-output --test-threads=1
```

记忆口诀：

> **名字选范围，线程管并行；输出参数看日志，ignore 默认不运行。**

## 只对某个文件进行测试
```sh
>cargo test -- --list
lifes::traitTemp::tests::dog_speaks: test
lifes::traitTemp::tests::cat_speaks: test

# 这就是找到对应的模块分支,进行测试
>cargo test --lib lifes::traitTemp:: -- --no-capture
>cargo test --lib lifes::traitTemp:: -- --no-capture --lib

# 也可以直接进入文件 测试, 锁定其中一个 func
>cargo test --test dog_speaks -- --no-capture
>cargo test --test dog_speaks functest -- --no-capture
```