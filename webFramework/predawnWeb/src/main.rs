/*
Predawn，一个类似 Spring Boot 的 web 框架。

在目前的 Rust 社区中，web 框架有很多，比如 axum、rocket、poem 等等，但是这些框架有一些问题，比如需要开发者添加一些模板启动代码、需要自己定义配置文件、没有自动依赖注入、集中式注册路由等等，有一些新的框架，如 loco、pavex 在尝试解决这些问题，但是它们的 API 设计对用惯了 Spring Boot 的我来说，还是不够顺手。

我决定写一个像 Spring Boot 那样的 web 框架。

要写 Spring Boot，先要写 Spring，所以我之前写了一个依赖注入框架 Rudi，目前已经发布到 0.8.1 版本，API 基本已经稳定了，可以用来写 web 框架了。

在 Rudi 的基础上，我完成了 Predawn 的第一个版本。
*/
use std::sync::Arc;

use async_trait::async_trait;
use predawn::{
    app::{run_app, Hooks},
    controller,
};
use rudi::Singleton;

struct App;

impl Hooks for App {}

#[tokio::main]
async fn main() {
    run_app::<App>().await;
}

#[async_trait]
trait Service: Send + Sync {
    fn arc(self) -> Arc<dyn Service>
    where
        Self: Sized + 'static,
    {
        Arc::new(self)
    }

    async fn hello(&self) -> String;
}

#[derive(Clone)]
#[Singleton(binds = [Service::arc])]
struct ServiceImpl;

#[async_trait]
impl Service for ServiceImpl {
    async fn hello(&self) -> String {
        "Hello, World!".to_string()
    }
}

#[derive(Clone)]
#[Singleton]
struct Controller {
    svc: Arc<dyn Service>,
}

#[controller]
impl Controller {
    #[handler(paths = ["/"], methods = [GET])]
    async fn hello(&self) -> String {
        self.svc.hello().await
    }
}