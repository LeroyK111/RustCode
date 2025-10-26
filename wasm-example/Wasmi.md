# Wasmi 的新的超快的执行引擎
经过数月的研究、开发和质量保证，Wasmi 有史以来最重要的更新终于准备好投入生产使用。

Wasmi 是一种高效且多功能的 WebAssembly (Wasm) 解释器，专注于嵌入式环境。它是插件系统、云主机和智能合约执行引擎的绝佳选择。

Wasmi 在最大努力的实现 Wasmtime API，使其成为理想的运行时。

通过 cargo install wasmi_cli 安装 Wasmi 的 CLI 工具，或通过 wasmi crate 将其用作库。

Wasmi v0.32 配备了新的执行引擎，利用基于寄存器的字节码将其执行性能提高了多达 5 倍。此外，由于惰性编译和其他新技术，其启动性能也提高了几个数量级。

文中做了一系列的 benchmark, 结论如下：

到目前为止，Wasmi 和 Wasm3 的表现最好，因为它们具有惰性编译功能。正如预期的那样，优化基于 JIT 的 Wasm 运行时（如 Wasmtime 和 Wasmer）在这种情况下表现较差。专为快速启动而设计的单通道 JIT，例如 Winch 和 Wasmer Singlepass，速度也明显较慢。尽管也使用了懒惰的翻译，但 Stitch 的翻译性能并不理想。然而，需要注意的是，Winch 和 Stitch 仍处于开发的实验阶段，改进是可以预期的。


