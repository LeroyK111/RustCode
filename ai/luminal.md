# Luminal是一个深度学习库，它使用可组合编译器来实现高性能。

功能特性：

速度

Luminal可以在m系列macbook上以每秒15-25个token的速度运行Q8 Llama 38b。我们的目标是成为任何设备上任何模型最快的ML框架。

简单

luminal的核心始终是最小的。应该有可能在一个下午了解整个核心库。

RISC-style架构

luminal中的所有内容都可以归结为11个基本操作：

Unary - Log2, Exp2, Sin, Sqrt, Recip

Binary - Add, Mul, Mod, LessThan

Other - SumReduce, MaxReduce, Contiguous


这些操作足以支持transformers，卷积神经网络等。

原生

当前的机器学习生态系统过于分散，解决方案不是另一个抽象层。Luminal是用rust编写的，并直接与CUDA / Metal api交互。没有间接或抽象、docker容器或虚拟环境。只是一个静态连接的rust crate。