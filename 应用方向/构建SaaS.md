# 为什么用Rust？

为项目选择合适的语言从来没有一个放之四海而皆准的决定。关于Meteroid项目的背景：
他们是一个6人的团队，几乎没有Rust经验，但有丰富的Scala/Java背景，构建数据密集型应用程序的经验。

Meteroid是一个计费平台的SaaS平台，非常注重分析、实时数据和可操作性。

后端完全使用Rust(分为2个模块和几个worker)，并使用gRPC-web与React前端通信。

因此，有一些不可协商的需求，它们恰好非常适合Rust：性能、安全性和并发性。Rust实际上消除了所有与内存管理相关的bug，它的并发原语非常吸引人(并且没有让人失望)。

在SaaS中，当处理敏感或关键任务时，所有这些特性都特别有价值。它显著减少了内存使用，这也是构建可扩展和可持续平台的主要好处。

## 经验1：学习曲线是真实存在的

学习Rust并不像学习另一门语言。所有权、借用和生命周期等概念最初可能会让人望而生畏，这使得原本微不足道的代码非常耗时。

尽管这个生态系统是令人愉快的(稍后会详细介绍)，但有时不可避免地需要编写较低级别的代码。

例如，考虑一个相当基础的API中间件(Tonic/Tower)，它只是简单地报告计算持续时间：

```rust
impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for MetricService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = BoxError>
        + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let started_at = std::time::Instant::now();
        let sm = GrpcServiceMethod::extract(request.uri());

        let future = inner.call(request);

        ResponseFuture {
            future,
            started_at,
            sm,
        }
    }
}

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    future: F,
    started_at: Instant,
    sm: GrpcServiceMethod,
}

impl<F, ResBody> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<ResBody>, BoxError>>,
{
    type Output = Result<Response<ResBody>, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let res = ready!(this.future.poll(cx));
        let finished_at = Instant::now();
        let delta = finished_at.duration_since(*this.started_at).as_millis();
        // this is the actual logic
        let (res, grpc_status_code) = (...) 

        crate::metric::record_call(
            GrpcKind::SERVER,
            this.sm.clone(),
            grpc_status_code,
            delta as u64,
        );

        Poll::Ready(res)
    }
}

```

除了泛型类型、泛型生命周期和trait约束之外，还需要为简单的服务中间件编写自定义的Future实现。

学习曲线可能因背景而异，如果习惯了JVM处理繁重的工作，并且像他们一样使用更成熟、更广泛的生态系统，那么可能需要更多的努力来理解Rust的独特概念和范式。

然而，一旦掌握了这些概念和原语，它们就会成为令人难以置信的强大工具，即使偶尔需要编写一些样板文件或宏，也能提高工作效率。

值得一提的是，谷歌已经在相当短的时间内成功地将团队从Go和C++过渡到Rust，并取得了积极的成果。

为了使学习曲线更平滑，可以考虑以下几点：

从头到尾阅读官方Rust Book，不要跳过章节，理解这些复杂的概念会变得容易得多。



练习，练习，再练习！通过Rust练习来建立肌肉记忆，并建立Rust心态。



参与Rust社区，他们是一群不可思议的人，总是愿意伸出援助之手。



利用GitHub的搜索功能来查找和学习其他项目。生态系统仍在不断发展，与他人合作是必不可少的(只是要注意许可并始终做出贡献)。


## 经验2：生态系统仍在成熟

Rust中的底层生态系统确实令人难以置信，拥有被社区广泛采用的精心设计和维护的库。这些库为构建高性能和可靠的系统奠定了坚实的基础。

但是，随着技术栈的增加，事情可能会变得稍微复杂一些。

例如，在数据库生态系统中，虽然存在用于关系数据库的sqlx和diesel等优秀crate，但对于许多异步或非关系数据库客户端，情况就更加复杂了。这些领域的高质量库，即使被大公司使用，通常也只有一个维护者，从而导致开发速度变慢和潜在的维护风险。

对于分布式系统原语，挑战更加明显，可能需要实现自己的解决方案

## 经验3：文档存在于代码中

许多库都有非常完善的方法文档，并在代码注释中提供了全面的示例。如果有疑问，请深入研究源代码并进行探索。通常会找到所寻求的答案，并对库的内部工作有更深入的了解。

虽然带有使用指南的外部文档仍然很重要，可以节省开发人员的时间和挫败感，但在Rust生态系统中，必要时准备好深入研究代码是至关重要的。

比如docs.rs网站为公共Rust crate提供了访问基于代码的文档的便捷方式。或者，可以使用cargo doc在本地为所有依赖项生成文档。

不用说，另一个有用的技巧是寻找示例(大多数库都有一个/examples文件夹)和其他使用这个库的项目，并参与这些社区。它们总是为如何使用库提供有价值的指导，并且可以作为自己项目实现的起点。

## 经验4：不要追求完美

当开始使用Rust时，人们总是希望尽可能地写出最习惯的、性能最好的代码。然而，大多数情况下，以简单和效率为优先。
例如，使用clone()或Arc在线程之间共享数据可能不是最节省内存的方法，但它可以极大地简化代码并提高可读性。只要意识到性能是否是主要的影响，并做出明智的决策，优先考虑简单性是完全可以接受的。

记住，过早优化是万恶之源。首先专注于编写干净、可维护的代码，然后在必要时进行优化。不要尝试微优化(除非真的需要)。Rust的强类型系统和所有权模型已经为编写高效和安全的代码提供了坚实的基础。

当需要优化性能时，请关注关键路径，并使用perf和flamgraph等分析工具来识别代码中真正的性能热点。对于工具和技术的全面概述，推荐《Rust Performance Book》。

## 经验5：错误也可以是好事

Rust的错误处理非常优雅，使用Result类型和?操作符鼓励显式错误处理和传播。然而，这不仅仅是处理错误，它还涉及提供具有可跟踪的清晰和信息丰富的错误消息。

Meteroid团队使用了thiserror库，它简化了使用错误消息创建自定义错误类型的过程。

在大多数Rust用例中，不太关心底层错误类型的栈跟踪信息，而更喜欢将其直接映射到域内的错误信息类型。
```rust
#[derive(Debug, Error)]
pub enum WebhookError {
    #[error("error comparing signatures")]
    SignatureComparisonFailed,
    #[error("error parsing timestamp")]
    BadHeader(#[from] ParseIntError),
    #[error("error comparing timestamps - over tolerance.")]
    BadTimestamp(i64),
    #[error("error parsing event object")]
    ParseFailed(#[from] serde_json::Error),
    #[error("error communicating with client : {0}")]
    ClientError(String),
}
```




花时间制作干净且信息丰富的错误消息可以极大地增强开发人员的体验并简化调试。这是一个小的努力，但产生显著的长期效益。

然而，有时保留完整的错误链是很有意义的，并需要在此过程中添加额外的上下文信息。

Meteroid团队目前正在试验error-stack，这是一个由hash.dev维护的库，它可以附加额外的上下文并将其保存在整个错误树中。