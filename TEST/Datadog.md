# Datadog 将静态分析器从 Java 迁移到 Rust, 分析时间减少了三倍
Codiga 加入 Datadog 后，作者团队需要将静态分析器整合到 Datadog，但遇到扩展语言支持和解析速度慢的问题。

最终选择使用Rust进行迁移， 在迁移过程中， 主要挑战是理解 Copy 和 Clone trait、借用检查器以及并行处理。

迁移后，分析时间减少了三倍，且不再依赖 JVM，分析器可以嵌入 IDE 实时反馈。