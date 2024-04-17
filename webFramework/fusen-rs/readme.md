fusen-rust是一个高性能，轻量级的微服务框架，通过使用Rust宏来解决目前主流rpc框架使用复杂，性能低等问题，不需要通过脚本和脚手架生成RPC调用代码，通过宏来进行编译期"反射"来实现高性能的调用，满足RPC调用的简易性，同时支持Dubbo3,SpringCloud微服务生态可以与Java项目进行服务注册发现与互相调用.

https://github.com/kwsc98/fusen-rs

目前Rust主流RPC和Web框架都是通过脚本来生成相应的服务调用代码，来达到类似RPC调用的方式，但是使用上却存在低灵活性，不方便修改的问题，本项目就是通过Rust宏来实现编译期“反射”即实现了类似于Java反射代理的方式，又实现了零抽象开销的高性能调用代码。

快速开始
Common InterFace
```rust
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ReqDto {
    pub str: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResDto {
    pub str: String,
}

#[fusen_trait(package = "org.apache.dubbo.springboot.demo")]
#[asset(spring_cloud = "service-provider")]
pub trait DemoService {
    async fn sayHello(&self, name: String) -> String;
    
    #[asset(path="/sayHelloV2-http",method = POST)]
    async fn sayHelloV2(&self, name: ReqDto) -> ResDto;

    #[asset(path="/divide",method = GET)]
    async fn divideV2(&self, a: i32, b: i32) -> String;
}
```
Server

```rust
#[derive(Clone, Debug)]
struct DemoServiceImpl {
    _db: String,
}

#[fusen_server(package = "org.apache.dubbo.springboot.demo")]
impl DemoService for DemoServiceImpl {

    async fn sayHello(&self, req: String) -> FusenResult<String> {
        info!("res : {:?}", req);
        return Ok("Hello ".to_owned() + &req);
    }
    #[asset(path="/sayHelloV2-http",method = POST)]
    async fn sayHelloV2(&self, req: ReqDto) -> FusenResult<ResDto> {
        info!("res : {:?}", req);
        return Ok(ResDto {
            str: "Hello ".to_owned() + &req.str + " V2",
        });
    }

    #[asset(path="/divide",method = GET)]
    async fn divideV2(&self, a: i32, b: i32) -> FusenResult<String> {
        info!("res : a={:?},b={:?}", a, b);
        Ok((a + b).to_string())
    }
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    fusen_common::logs::init_log();
    let server = DemoServiceImpl {
        _db: "我是一个DB数据库".to_string(),
    };
    //支持多协议，多注册中心的接口暴露
    FusenServer::build()
        //初始化Fusen注册中心,同时支持Dubbo3协议与Fusen协议
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("fusen-service".to_owned()))
                .server_type(fusen_rs::register::Type::Fusen)
                .build()
                .boxed(),
        )
        //初始化SpringCloud注册中心
        .add_register_builder(
            NacosConfig::builder()
                .server_addr("127.0.0.1:8848".to_owned())
                .app_name(Some("service-provider".to_owned()))
                .server_type(fusen_rs::register::Type::SpringCloud)
                .build()
                .boxed(),
        )
        //同时兼容RPC协议与HTTP协议
        .add_protocol(Protocol::HTTP("8081".to_owned()))
        .add_protocol(Protocol::HTTP2("8082".to_owned()))
        .add_fusen_server(Box::new(server))
        .run()
        .await;
}
```
Client
```rust
lazy_static! {
    static ref CLI_FUSEN: FusenClient = FusenClient::build(
        NacosConfig::builder()
            .server_addr("127.0.0.1:8848".to_owned())
            .app_name(Some("fusen-client".to_owned()))
            .server_type(fusen_rs::register::Type::Fusen)
            .build()
            .boxed()
    );
    static ref CLI_DUBBO: FusenClient = FusenClient::build(
        NacosConfig::builder()
            .server_addr("127.0.0.1:8848".to_owned())
            .app_name(Some("dubbo-client".to_owned()))
            .server_type(fusen_rs::register::Type::Dubbo)
            .build()
            .boxed()
    );
    static ref CLI_SPRINGCLOUD: FusenClient = FusenClient::build(
        NacosConfig::builder()
            .server_addr("127.0.0.1:8848".to_owned())
            .app_name(Some("springcloud-client".to_owned()))
            .server_type(fusen_rs::register::Type::SpringCloud)
            .build()
            .boxed()
    );
}

#[tokio::main(worker_threads = 512)]
async fn main() {
    fusen_common::logs::init_log();
    //进行Fusen协议调用HTTP2 + JSON
    let client = DemoServiceClient::new(&CLI_FUSEN);
    let res = client
        .sayHelloV2(ReqDto {
            str: "world".to_string(),
        })
        .await;
    info!("rev fusen msg : {:?}", res);

    //进行Dubbo3协议调用HTTP2 + GRPC
    let client = DemoServiceClient::new(&CLI_DUBBO);
    let res = client.sayHello("world".to_string()).await;
    info!("rev dubbo3 msg : {:?}", res);

    //进行SpringCloud协议调用HTTP1 + JSON
    let client = DemoServiceClient::new(&CLI_SPRINGCLOUD);
    let res = client.divideV2(1, 2).await;
    info!("rev springcloud msg : {:?}", res);
}
```