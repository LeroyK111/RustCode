Rust 检测器分为了四大类：linter、静态检测器、动态检测器和形式化验证器。对于每款检测器,我都包括了以下信息：代码和论文链接、所使用的中间表示(IR)、可检测的错误类型、底层技术方法，以及当前的维护状态。


# Kani
Kani 是一个开源验证工具，使用模型检查来分析 Rust 程序。Kani 对于验证 Rust 中的 unsafe 代码块特别有用。


# Sniffnet


Sniffnet 是一个基于Rust的网络监控工具，作者已经开发了将近两年时间。经过8个月的开发，Sniffnet v1.3.0版本终于发布。
新增特性：

支持导出PCAP文件。
新增对ICMP的支持。
检测更多上层服务。
新的缩略图模式。
完全自定义应用颜色调板。
文档优化

