
# 容器的构建模块

为了理解容器，我们需要分解它们的核心组件：

1，Namespaces(命名空间)：这个Linux特性提供隔离。不同类型的命名空间(PID、NET、IPC、MNT、UTS)确保一个容器中的进程不能看到或影响另一个容器中的进程。

2，Control Groups(控制组)：它们用于限制、说明和隔离进程组的资源使用情况(CPU、内存、磁盘I/O等)。

3，Union File Systems(合并文件系统)：这允许覆盖多个文件系统，使只读基本映像具有附加的读写层成为可能。

![](../learning/src/objInfo/assets/Pasted%20image%2020240725201039.png)


## Docker如何使用这些组件

Docker将应用程序打包成镜像，然后作为容器运行。以下是运行Docker容器时发生的事情的简化摘要：

1，镜像加载：Docker从它的存储库中加载镜像。

2，文件系统隔离：使用UnionFS，Docker创建一个特定于容器的文件系统。

3，进程隔离：Docker使用命名空间来提供进程、网络和文件系统隔离。

4，资源管理：Docker使用cgroups来管理分配给容器的资源。

5，容器执行：最后，Docker启动容器化进程。

![](../learning/src/objInfo/assets/Pasted%20image%2020240725201057.png)

## rs-docker

从理论上理解这些概念是一回事，但看到它们的行动是另一回事。这就是为什么创建了rs-docker，这是一个模仿Docker核心功能的教育工具，它非常适合学习和轻量级使用。

主要特点

1，文件系统隔离：使用chroot实现隔离的文件系统

2，进程隔离：使用ushare单独运行进程。空间。

3，注册镜像：支持从注册表中提取和存储镜像。

4，跨平台兼容性：可以在Linux上工作，并且可以使用Docker在Windows/MacOS上运行。

![](../learning/src/objInfo/assets/Pasted%20image%2020240725201158.png)

让我们介绍一下如何开始使用这个工具来更好地理解容器化。

1.克隆存储库

```
git clone https://github.com/dpouris/rs-dockercd rs-docker
```


2.运行脚本

在Linux或Windows (WSL)上，构建并运行：

```
cargo build - release./target/release/rs-docker run ubuntu:latest echo "hello world"
```

在MacOS上，使用提供的脚本：

```
chmod +x ./docker.sh./docker.sh run ubuntu:latest echo "hello world"
```

这将拉出最新的ubuntu映像，并在隔离的容器环境中运行echo " hello world "命令，演示文件系统和进程隔离的实际效果。

为什么要使用这个工具？

教育价值：加深对集装箱化工作原理的了解

动手学习：在没有Docker复杂性的情况下，尝试核心容器特性。

轻量级用例：将其用于简单、孤立的环境，而不需要完整的Docker。

## 总结

容器在现代软件开发中是一个强大的工具，提供了很高的效率和一致性。通过了解它们的内部工作原理，可以更好地利用它们的潜力并更有效地解决问题。rs-docker对于任何想要深入了解容器世界的人来说都是一个很好的起点。