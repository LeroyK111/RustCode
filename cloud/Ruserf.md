# 可嵌入式的、高度可定制、适应性强、与运行时无关且 WASM/WASI 友好的去中心化解决方案，用于服务发现和编排，具有轻量级、高可用性和容错能力。

将 HashiCorp 的 serf 移植并改进为 Rust版本。

支持TCP/TLS & UDP 和 QUIC & QUIC传输
支持多种异步运行时
Rust实现采用组件式架构, 利用trait系统可以轻松插拔定制化不同的组件

项目地址: https://github.com/al8n/ruserf