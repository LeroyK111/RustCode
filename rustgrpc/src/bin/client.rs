pub mod hello {
    tonic::include_proto!("hello");
}

use hello::{greeter_client::GreeterClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}


/*

执行以下命令运行server端：
cargo run --bin rustgrpc
执行以下命令运行client端：
cargo run --bin client
如果一切正常，应该在client的终端中看到以下内容：
RESPONSE=Response { metadata: MetadataMap { headers: {"content-type": "application/grpc", "date": "", "grpc-status": "0"} }, message: HelloReply { message: "Hello Tonic!" }, extensions: Extensions }

*/