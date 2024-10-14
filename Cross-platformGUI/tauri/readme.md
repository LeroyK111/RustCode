# Tauri 是一个为所有主流桌面平台 (macOS、Linux、Windows) 和移动设备平台 (iOS、Android) 构建轻量级二进制文件的框架。


在 Tauri 应用程序中，开发者可以使用熟悉的 Web 技术栈编写前端页面。它在操作系统 WebView 中运行，并与主要用 Rust 编写的应用程序核心进行通信。

![](../../learning/src/objInfo/assets/Pasted%20image%2020241014185926.png)

也就是说，开发者可以集成任何可编译为 HTML、JavaScript 和 CSS 的前端框架来编写 UI，同时在需要时利用 Rust、Swift 和 Kotlin 等语言来构建后端逻辑。

与因体积庞大而饱受诟病的 Electron 相比，Tauri 更加轻量、性能更好，提供了一种更现代、更安全且资源效率更高的方法来构建跨平台桌面应用。

下面是 Tauri v1 和 Electron 的技术特性对比：

![](../../learning/src/objInfo/assets/Pasted%20image%2020241014185943.png)
近日，Tauri 正式发布了 2.0 稳定版。

开发团队表示，“**Tauri v2 是支持跨平台开发的一个重大里程碑，开发桌面和移动应用程序从未如此简单**。你可以将现有的桌面程序无缝迁移到移动设备，并获得原生 API 和 Tauri CLI 的出色开发者体验。”
![](../../learning/src/objInfo/assets/Pasted%20image%2020241014185958.png)

支持移动操作系统无疑是 Tauri v2 最值得期待的新特性。Tauri v1 实现了在桌面操作系统中使用单一的 UI 代码库，而现在则扩展到了 iOS 和 Android。

据介绍，Tauri 团队调查并尝试了不同的移动支持解决方案，最终决定使用两大移动操作系统的原生语言（Swift 和 Kotlin）为 Rust 代码构建界面，并允许开发者使用这些语言编写部分功能。

这意味着开发者可以**复用 Swift 或 Kotlin 应用程序中与系统交互的现有逻辑**，并将其暴露给 Rust 或前端。  

团队还介绍称，他们在 2022 年 6 月发布了 [Tauri 1.0](http://mp.weixin.qq.com/s?__biz=MjM5NzM0MjcyMQ==&mid=2650148081&idx=1&sn=3a22c072cb3cc3ccf1c1b9be53bbf39f&chksm=bed9d79f89ae5e8961d51fc6f8e7bb8c36c2f0f5a2b3d191ad45e624b03d1e0797c488a1622f&scene=21#wechat_redirect)，该版本对桌面操作系统市场以及如何构建跨平台应用产生了重大影响。

可以看到，从 Tauri v1 到 v2，其 Slogan 也变得更简洁、聚焦：**_“创建轻量、快速、安全的跨平台应用”_**。

最后看看 Tauri 2.0 主要新特性和改进：

- 移动支持：添加了对 iOS 和 Android 的支持。
    
- 多 WebView 支持：支持在应用中使用多个 WebView，通过不稳定特性标志启用。
    
- rustls-tls 特性标志：添加了对 rustls TLS backend 的支持。
    
- 窗口阴影选项：创建 WebView 窗口时添加了设置窗口阴影的选项。
    
- IPC 模块：添加了新的 IPC 模块，支持原始数据传输。
    
- 文件系统模块：添加了新的文件系统模块和 API。
    
- 自动启动：支持应用在系统启动时自动启动。
    
- 条形码扫描器：允许移动应用使用相机扫描条形码。
    
- 生物识别：在 Android 和 iOS 上提示用户进行生物识别验证。
    
- 剪贴板访问：读取和写入系统剪贴板。
    
- 命令行界面：解析命令行界面的参数。
    
- 深度链接：将 Tauri 应用程序设置为 URL 的默认处理程序。
    
- 对话框：打开和保存文件的原生系统对话框。
    
- 全局快捷键：注册全局快捷键。
    
- HTTP 客户端：使用 Rust 编写的 HTTP 客户端。
    
- 本地主机：在生产应用程序中使用本地主机服务器。
    
- 日志记录：可配置的日志记录。
    
- NFC：在 Android 和 iOS 上读取和写入 NFC 标签。
    
- 通知：向用户发送原生通知。
    
- 操作系统信息：读取操作系统信息。
    
- 持久作用域：在文件系统上持久化运行时作用域更改。
    
- 定位器：将窗口移动到常见位置。
    
- 进程访问：访问当前进程。
    
- Shell：访问系统 Shell，使用默认应用程序管理文件和 URL，并生成子进程。
    
- 单实例：确保 Tauri 应用只有一个实例在运行。
    
- SQL：为前端提供了与 SQL 数据库通信的接口。
    
- 存储：持久化键值存储。
    
- 加密数据库：提供加密、安全的数据库。
    
- 系统托盘：添加了系统托盘支持。
    
- 自动更新：为 Tauri 应用程序提供应用内更新。
    
- 文件上传：通过 HTTP 上传文件。
    
- WebSocket：使用 Rust 客户端在 JavaScript 中打开 WebSocket 连接。
    
- 窗口自定义：自定义窗口状态，包括窗口大小和位置。




