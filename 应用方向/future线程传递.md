# 如何解决“future不能安全地在线程之间发送”的问题？

Rust应用程序通常使用异步库，如Tokio和Actix。这些库为异步I/O和并行计算等提供了有力的支持。然而，不同的异步库在一起使用时，有时会出现问题。

当在Tokio运行的异步函数中使用Actix client时，可能会发生“error: future不能安全地在线程之间发送”的错误，这在使用Tokio和Actix库时是一个常见的问题。今天，我们来看看如何解决这个问题。

让我们从一个简单的代码示例开始，它只适用于Actix，不会产生任何问题：

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use awc::Client;

#[actix_rt::main]
async fn main()  {
    actix_rt::spawn(async {
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/hello").route(web::get().to(ok)))
        })
            .bind("127.0.0.1:8080")?
            .run()
            .await
    });

    let client = Client::new();
    let url = "http://127.0.0.1:8080/hello";
    let ret =  client.get(url).send().await.unwrap().body().await.unwrap();
    println!("{:?}", ret);
}

async fn ok() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("OK")
}
```
在这段代码中，我们使用Actix创建一个HTTP服务器，并使用Actix client向它发出GET请求。一切都很顺利，但是当我们试图在Tokio运行的异步函数中使用Actix client时，问题就开始了。

当我们尝试在Tokio运行时中调用Actix client时，我们会遇到“error: future不能安全地在线程之间发送的错误。async block创建的future不是Send。类型 awc::Client 不是Send”。这是因为Actix client不是Send，这意味着它不能在线程之间安全地传递。

下面是导致此错误的示例代码：

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use awc::Client;

#[actix_rt::main]
async fn main()  {
    actix_rt::spawn(async {
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/hello").route(web::get().to(ok)))
        })
            .bind("127.0.0.1:8080")?
            .run()
            .await
    });

    let r = tokio::spawn(async move {
        let client = Client::new();
        let url = "http://127.0.0.1:8080/hello";
        client.get(url).send().await.unwrap().body().await.unwrap()
    }).await.unwrap();

    println!("{:?}", r);
}

async fn ok() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("OK")
}
```
为了解决这个问题并使代码在Tokio中安全使用，我们可以使用来自Tokio的Oneshot机制。这种机制允许我们封装Actix client的输出，并在线程之间安全地传递它。

下面是用Oneshot用来解决这个问题的示例代码：
```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use awc::Client;

#[actix_rt::main]
async fn main()  {
    actix_rt::spawn(async {
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/hello").route(web::get().to(ok)))
        })
            .bind("127.0.0.1:8080")?
            .run()
            .await
    });

      let (sender, receiver) = tokio::sync::oneshot::channel();

    actix_rt::spawn(async move {
        let client = Client::new();
        let url = "http://127.0.0.1:8080/hello";
        let _ = sender.send(client.get(url).send().await.unwrap().body().await.unwrap());
    });

    let r = tokio::spawn(async move {
        receiver.await.unwrap()
    }).await.unwrap();

    println!("{:?}", r);
    std::mem::forget(runtime);
}

async fn ok() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("OK")
}
```
