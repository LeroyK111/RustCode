/*
创建pingora服务器

首先，让我们创建一个pingora服务器。pingora服务器是一个可以承载一个或多个服务的进程。pingora服务器负责配置和CLI参数解析、守护进程、信号处理以及正常重启或关闭。

首选的用法是在main()函数中初始化Server，并使用run_forever()生成所有运行时线程并阻塞主线程，直到服务器准备退出。
*/


use pingora::server::Server;

fn main() {
    let mut server = Server::new(None).unwrap();
    server.bootstrap();
    server.run_forever();
}


/*
创建负载均衡器代理

接下来，让我们创建一个负载均衡器。我们的负载均衡器持有一个静态的上游ip列表。pingora-load-balancing crate已经为LoadBalancer结构提供了常用的选择算法，如轮询和哈希。

为了使服务器成为代理，我们需要为它实现ProxyHttp特性。任何实现ProxyHttp特性的对象本质上都定义了在代理中如何处理请求。ProxyHttp特性中唯一需要实现的方法是upstream_peer()，它返回请求应该被代理到的地址。

在upstream_peer()的主体中，让我们使用select()方法让LoadBalancer在上游ip之间进行轮询。在本例中，我们使用HTTPS连接到后端，因此在构造Peer对象时，我们还需要指定到use_tls并设置SNI。
*/

use std::sync::Arc;

use async_trait::async_trait;
use pingora::{
    lb::{selection::RoundRobin, LoadBalancer},
    proxy::{ProxyHttp, Session},
    server::Server,
    upstreams::peer::HttpPeer,
    Result,
};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// 对于这个小示例，我们不需要上下文存储
    type CTX = ();
    fn new_ctx(&self) {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256)
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, true, "one.one.one.one".to_string()));
        Ok(peer)
    }
}

/*
为了让1.1.1.1后端接受我们的请求，主机Header必须存在。添加此报头可以通过upstream_request_filter()回调来完成，该回调在与后端建立连接之后和发送请求报头之前修改请求报头。
*/

#[async_trait]
impl ProxyHttp for LB {
   ......

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();
        Ok(())
    }
}


/*
执行cargo run，运行我们的负载均衡器。

要测试它，只需使用以下命令多次向服务器发送请求：
curl 127.0.0.1:6188 -svo /dev/null
负载均衡器日志如下：
upstream peer is: Backend { addr: Inet(1.0.0.1:443), weight: 1 }
upstream peer is: Backend { addr: Inet(1.1.1.1:443), weight: 1 }
upstream peer is: Backend { addr: Inet(1.0.0.1:443), weight: 1 }
upstream peer is: Backend { addr: Inet(1.1.1.1:443), weight: 1 }
*/
fn main() {
    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443"]).unwrap();

    let mut lb = http_proxy_service(&server.configuration, LB(Arc::new(upstreams)));
    lb.add_tcp("0.0.0.0:6188");

    server.add_service(lb);

    server.run_forever();
}