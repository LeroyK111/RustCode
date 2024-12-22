# rust 2.0 版本

函数着色是编程界的一个众所周知的问题，Bob Nystrom在2015年发表了一篇文章：
`What Color is Your Function？“红色(异步)函数”不能调用“蓝色(同步)函数”，反之亦然。`

在过去的日子里，你必须手工编写事件循环和状态机，然后花几年时间修复错误、崩溃和漏洞。在Rust中，使用async，它可以正常工作。编译器负责一切，为你生成状态机并确保它们是正确的。不是很棒，是棒极了!

那么，如果async/await实际上可以帮助开发人员更有效地使用机器，为什么称它为一个代价非常高昂的错误呢？

因为，如果执行不当，它会分裂生态系统和库：有同步库和异步库。开发人员对何时使用函数的异步版本或同步版本感到困惑。库开发人员花了很多努力来适应这两个世界，事件循环被同步调用阻塞。

`具有讽刺意味的是，由于单线程性质，JavaScript可能是唯一一种正确使用异步的语言：所有I/O调用都是异步的，因此语言和库在默认情况下都是异步的。`

Rust在2019年11月发布async/await之后，Rust并没有在async上全速前进，我们现在陷入了一个奇怪的困境，很多(可能是大多数)开发人员每天都在使用异步，async和await是语言的关键字，但是没有官方的异步运行时，运行时甚至不兼容，更糟糕的是，标准库不提供任何异步I/O功能！

async，它是语言中的一等结构，但在生态系统中却是二等公民。难怪每个人都很困惑，不知道该怎么解决实际问题。

`也许未来唯一的解决方案是Rust 2.0将异步作为真正的一等公民，就像JavaScript一样，其中每个I/O函数都将是异步的。有这么多聪明的人在Rust上工作，我们可以找到一些主要向后兼容的解决方案，这将使每个使用这门语言的人更轻松。`

## rust 2024 新特性

支持异步闭包和发送边界；
通过稳定一些已发布的特性，使rust在Linux内核中的应用畅通无阻；
Cargo脚本，允许你用rust代码和嵌入的依赖关系创建一个单独的文件；
稳定并行前端编译；
在处理引用计数数据时，可以将编译时间提高20%，减少语法开销；

### Generators(生成器)

Rust 2024版中第一个令人兴奋的变化是保留Gen关键字，以便将来可以添加生成器。生成器提供了一种使用命令式语法创建迭代器的方法，或者换句话说，就是描述一系列动作，它的最大的好处是可以大大简化复杂迭代器的创建。假设我们想要使用 the Civ of Athos 算法实现一个产生质数的迭代器。


### Polonius(借用检查算法)

Polonius是Rust的下一代借用检查算法，它是当前借用检查器的改进版本，解决了一些常见的限制。例如，假设我们有一个函数，它接受HashMap和键作为输入，它返回一个对值的可变引用。如果键存在则返回对它的可变引用；如果键不存在，则插入默认值并返回对它的可变引用。返回Some中对value的可变引用要求Map的可变引用一直存在到函数结束。

## 2024年
### Connectivity

- **tokio**: 异步运行时，支持 HTTP 和其他协议。
    
- **axum**: 用于构建 HTTP(S) 服务器，支持路由、状态共享等。
    
- **tower-http**: HTTP 中间件，例如认证和请求验证。
    
- **reqwest**: HTTP 客户端，支持 rustls。
    
- **warp**: 用于构建轻量级 HTTP 服务器。
    
- **prost** 和 **tonic**: 用于 protobuf 和 gRPC。
    
- **lapin**: RabbitMQ 客户端。
    

### Serialization & Data

- **serde** 和 **serde_json**: 序列化和 JSON 处理。
    
- **bincode**: 二进制序列化，用于高效存储。
    
- **humantime-serde**: 支持人类可读的时间格式序列化。
    

### Error Handling

- **thiserror**: 用于库的错误处理。
    
- **anyhow**: 用于应用的错误处理。
    

### Testing

- **rstest**: 参数化测试和 fixtures。
    
- **criterion**: 性能基准测试工具。
    

### Utilities

- **rustc-hash** 和 **sha1_smol**: 非加密的高效哈希。
    
- **tikv-jemallocator**: 用于优化内存分配。
    
- **uuid**: 支持 UUID 生成和解析。
    
- **chrono**: 时间和日期处理。
    
- **derivative**: 自定义派生的 trait 实现。
    
- **image**: 图像处理。
    

### CLI

- **argh** 和 **clap**: CLI 解析。
    

### Logging, Tracing and Metrics

- **tracing**: 结构化日志和跟踪。
    
- **prometheus**: 用于监控指标。
    

### SQL & ORMs

- **sea-orm** 和 **sea-query**: ORM 和查询构建工具。
    
- **sqlx**: 异步数据库操作。
    

### Vectors, Arrays, ML

- **ndarray** 和 **nalgebra**: 数组和线性代数。
    
- **half**: 支持 f16 类型。
    
- **approx**: 浮点数近似比较。
    
- **ort**: ONNX 运行时，用于机器学习推理。
    

### Deprecated or Alternative Crates

- **lazy_static**: 已被 **LazyLock** 取代。
    
- **once_cell**: 可使用标准库中的 **OnceLock**。
    
- **async-trait**: 在某些情况下仍需使用，但已逐渐被 async 函数支持替代。