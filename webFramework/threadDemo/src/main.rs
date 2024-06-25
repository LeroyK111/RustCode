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