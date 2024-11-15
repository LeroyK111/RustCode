### Matchbox - 类似UDP的不可靠、无序的点对点网络连接

Matchbox项目旨在为Rust语言的原生应用和WebAssembly应用程序实现类似UDP的不可靠、无序的点对点网络连接,以支持低延迟的多人在线游戏。

Matchbox包括几个主要部分:

- matchbox_socket: 一个封装了WebRTC的Socket抽象层,支持不可靠和可靠的数据通道。
    
- matchbox_signaling: 一个信令服务器库及示例。
    
- matchbox_server: 一个现成的全网状信令服务器。
    
- bevy_matchbox: 将matchbox_socket集成到Bevy游戏引擎中。
    

该仓库还提供了一些使用示例,包括一个简单的通信循环、使用Bevy和GGRS的浏览器游戏demo等。

Matchbox的工作原理是先通过信令服务器协调对等体之间的连接,之后对等体就可以直接相互发送数据,不再经过信令服务器。

该项目欢迎贡献和问题反馈,采用双重MIT和Apache 2.0许可证。感谢Ernest Wong的Dango Tribute实验对该项目的启发。

https://github.com/johanhelsing/matchbox