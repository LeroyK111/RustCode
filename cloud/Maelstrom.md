# Maelstrom：一个封闭的、集群的 Rust 测试运行器（而且速度很快）

Maelstrom 是一个开源 Rust 测试运行器，构建在通用集群作业运行器之上。Maelstrom 将您的 Rust 测试打包到密封的微容器中，然后将它们分发到任意大的测试运行器集群上或在您的本地计算机上运行。您可以使用 Maelstrom 来运行测试，因为：

这很容易。Maelstrom 可以作为货物测试的直接替代品，因此在大多数情况下，它都能正常工作。
这是可靠的。Maelstrom 在自己的轻量级容器中密封地运行每个测试，消除了由测试间或隐式测试环境依赖性引起的混乱错误。
它是可扩展的。Maelstrom 可以作为集群运行。您可以添加更多工作机器以线性增加测试吞吐量。
它很快。在大多数情况下，即使不使用集群，Maelstrom 也比货物测试更快。
很干净。Maelstrom 有一个从头开始的无根容器实现（不依赖 Docker 或 RunC），经过优化以降低开销并快速启动。
这是生锈的。整个项目是用 Rust 编写的。
我们从 Rust 测试运行程序开始，但 Maelstrom 的底层作业执行系统是通用的。我们将在不久的将来添加对其他语言测试框架的支持。我们还为喜欢冒险的用户提供了运行任意作业的工具，可以使用命令行工具或基于 gRPC 的 SDK。

欢迎反馈和提问！感谢您尝试一下。

https://maelstrom-software.com/

https://github.com/maelstrom-software/maelstrom