RIIR - hawkeye ，许可协议标头格式化工具
hawkeye 是一款许可协议标头格式化工具（license header formatter），用于格式化或者检查源文件中的许可协议标头，过去使用 Java 编写，近日已经完全用 Rust 进行了重写。

Cargo
hawkeye 可执行文件可以通过 Cargo 安装：

```sh
cargo install hawkeye

# check license headers
hawkeye check
# format license headers (auto-fix all files that failed the check)
hawkeye format
# remove license headers
hawkeye remove
```


