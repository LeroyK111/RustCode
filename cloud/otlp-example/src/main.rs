

use opentelemetry::trace::TraceError;
use rand::Rng;

/*
执行cargo run运行程序。

执行程序多次后，打开Jaeger UI：http://localhost:16686/，选择我们配置的服务名称：rust-otlp-basic。如图：
像这样使用OpenTelemetry还是有些复杂，在下一篇文章中，我们将引入Tracing crates，来简化OpenTelemetry的使用。
*/

fn gen_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

// 创建一个常量 Key
const RANDOM: Key = Key::from_static_str("random.value");

#[tokio::main]
async fn main() -> Result<(), TraceError> {
    // 设置全局传播器
    global::set_text_map_propagator(TraceContextPropagator::new());

    // 初始化跟踪程序
    let tracer = init_tracer().unwrap();

   // 启动一个新的活跃的跨度
   tracer.in_span("generating number", |cx| {
    let span = cx.span();
    let num = gen_number();
    span.add_event(
        "Generating Number".to_string(),
        vec![Key::new("number").i64(num.into())],
    );

    // 设置span属性
    span.set_attribute(RANDOM.i64(10));


    // 开始一个新的span
    tracer.in_span("generate another number", |cx| {
            let span = cx.span();
            let num = gen_number();
            span.add_event(
                "Generating Number".to_string(),
                vec![Key::new("number").i64(num.into())],
            )
        })
    });

    // 优雅地关闭跟踪器
    global::shutdown_tracer_provider();
    Ok(())
}

// 创建init_tracer函数来初始化OpenTelemetry跟踪器：

fn init_tracer() -> Result<trace::Tracer, TraceError> {
    // 初始化OTLP管道
    opentelemetry_otlp::new_pipeline()
        .tracing() // 创建OTLP跟踪管道
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic() // 创建GRPC层
                .with_endpoint("http://localhost:4317"), // GRPC OTLP Jaeger端口
        )
        // 跟踪配置
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                "rust-otlp-basic",
            )])),
        )
        .install_batch(runtime::Tokio) // 配置一个span导出器
}



