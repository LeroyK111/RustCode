# 构建消息代理服务器

消息代理服务器允许客户端为主题生成事件并订阅它们。它使用Warp作为HTTP和WebSocket服务器，使用Tokio作为异步运行时。

测试

执行如下命令运行消息代理服务器：
`cargo run --bin real-ime-message`
执行结果：
`Broker server running at http://127.0.0.1:3030`

然后打开一个新的命令行，执行如下命令运行WebSocket客户端：
`cargo run --bin ws_cli`
执行结果：
`WebSocket client connected`

向http://127.0.0.1:3030/produce/newtopic接口发送post请求，如图：

客户端接收到消息：
`WebSocket client connected`
`Received message: This is a new event`