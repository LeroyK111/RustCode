use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};

async fn hello() -> &'static str {
   "Hello world"
}

#[tokio::main]
async fn main() {
   // 配置跟踪
   // 构造一个订阅者，将格式化的跟踪信息打印到标准输出
   let subscriber = tracing_subscriber::FmtSubscriber::new();
   // 使用subscriber处理在此点之后发出的跟踪
   tracing::subscriber::set_global_default(subscriber).unwrap();

   // 允许每个IP地址最多有五个请求，每两秒钟补充一个
   // 我们将其装箱是因为Axum 0.6要求所有层都是克隆的，因此我们需要一个静态引用
   let governor_conf = Box::new(
       GovernorConfigBuilder::default()
           .per_second(2)
           .burst_size(5)
           .finish()
           .unwrap(),
   );

   let governor_limiter = governor_conf.limiter().clone();
   let interval = Duration::from_secs(60);
   // 一个单独的后台任务
   std::thread::spawn(move || {
       loop {
           std::thread::sleep(interval);
           tracing::info!("rate limiting storage size: {}", governor_limiter.len());
           governor_limiter.retain_recent();
       }
   });

   // 构建路由
   let app = Router::new()
       // `GET /` 
       .route("/", get(hello))
       .layer(GovernorLayer {
           // 我们可以泄漏它，因为它是一次性创建的
           config: Box::leak(governor_conf),
       });

    // 绑定地址
   let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
   tracing::debug!("listening on {}", addr);
   let listener = TcpListener::bind(addr).await.unwrap();
   axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
       .await
       .unwrap();
}