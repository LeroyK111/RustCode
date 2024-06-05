# rust GUI 开发库

Rust项目 Ratatui获得资助
Ratatui是tui-rs库的继承者，已经围绕Rust/TUI生态系统建立了一个持续增长的社区

## wgpu

wgpu框架，是一个跨平台、安全、纯 Rust 的图形 API。它可以运行在 Vulkan、Metal、D3D12 和 OpenGL ，以及 wasm 上的 WebGL2 和 WebGPU 之上。

它的API基于WebGPU 标准实现。


## Freya 0.2
我刚刚发布了 Freya v0.2 ，这是我的基于 Dioxus 和 Skia 的 Rust GUI 库。



## Zed 解析: Rope 和 SumTree
Zed是 Rust 构建的文本编辑器, 本文将介绍他的核心数据结构——Rope和SumTree。

Rope和传统字符串比较：

Rope是一种二叉树结构，每个叶节点储存一个字符串和其长度，而树上的其他节点则存储所有左子树叶节点长度的总和。与字符串相比，在编辑大型文件或进行频繁编辑时，Rope更内存和性能高效，因为可以避免大量内存分配和字符移动。
Zed的Rope实现——SumTree：

Zed没有选择典型的Rope实现，而是采用了SumTree，这是一种特殊的B+树，允许在O(log N)时间内进行高效的数据遍历。SumTree中的每个节点都包含一个摘要（Summary），这个摘要可以是任何信息，如文本的UTF-8和UTF-16的长度、行数等。
使用SumTree的好处：
SumTree不仅支持并发访问和多线程操作，还能快速生成文本的快照，非常适合进行异步保存、备份或多用户编辑等操作。

Zed中有超过20个功能使用了SumTree结构，如项目中的文件列表、git blame信息、聊天消息、诊断信息等。总结来说，SumTree作为Zed的核心组件，不但具备了常规Rope的优势，还赋予了Zed极高的性能和灵活性，使其成为一个高效的代码编辑器。

## tui-realm

ui-realm是一个用于ui和ratatui的框架，用于实现简化的终端用户界面，增加了使用具有属性和状态的可重用组件，就像在React中所做的那样。但这还不是全部：组件通过基于消息和事件的系统与ui引擎通信，提供实现更新例程的可能性，就像在Elm中发生的那样。此外，组件被组织在视图中，它管理挂载/卸载、焦点和事件转发。

这也解释了名称的原因：Realm代表React和Elm。

## gitu - git 的 tui 界面
灵感来自 Magit。操作方式沿用Magit的配置。

# ratatui 

