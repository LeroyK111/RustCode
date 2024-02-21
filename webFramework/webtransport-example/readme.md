# Rust WebTransport库

WebTransport是一种新的协议，用于实现客户端和服务器之间通过web进行低延迟、双向通信。它旨在通过提供更高效、更灵活的传输层来解决WebSocket协议的局限性。

WebTransport的优点：

低延迟：WebTransport旨在最大限度地减少延迟，使其适用于实时应用程序，如游戏，视频流和协同编辑。

双向通信：WebTransport允许在客户端和服务器之间同时交换数据，实现高效的双向通信，而不需要多个请求。

多路复用：使用WebTransport，多个流可以在单个连接上进行多路复用，从而减少开销并提高性能。

安全性：WebTransport受益于web平台提供的安全特性，包括传输加密和同源策略。 

```rust
测试

执行如下命令生成证书：
```rust
cargo run --bin gencert
```
在项目根目录下生成cert.pem和key.pem两个文件。

执行如下命令启动server：
```rust
cargo run --bin server
```
然后打开一个新的终端，执行如下命令运行client：


```rust
cargo run --bin client
这时，服务器端的输出信息如下：
INFO server: Server ready!
INFO Connection{id=0}: server: Waiting for session request...
INFO Connection{id=0}: server: New session: Authority: '[::1]:4433', Path: '/'
INFO Connection{id=0}: server: Waiting for data from client...
INFO Connection{id=0}: server: Accepted BI stream
INFO Connection{id=0}: server: Received (bi) 'HELLO' from client
INFO Connection{id=0}: server: Accepted UNI stream
INFO Connection{id=0}: server: Received (uni) 'WORLD' from client
INFO Connection{id=0}: server: Received (dgram) 'Hello, world!' from client
```