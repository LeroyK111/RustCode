# Rust中的分布式跟踪：OpenTelemetry

在这篇文章中，我们先简单介绍一下OpenTelemetry，然后，再来编写一个在Rust中使用OpenTelemetry的示例。

OpenTelemetry，也被简称为OTel，是一个开源可观察性框架，用于检测、生成、收集和导出测量数据，如跟踪、度量、日志。OTEL为你提供了一套独立的api、sdk、集成和收集器服务。如图：

![](../../learning/src/objInfo/assets/Pasted%20image%2020240513202833.png)

下面，让我们来看一下OpenTelemetry的关键概念。

OpenTelemetry介绍

核心术语

跟踪(Trace)：

- 描述请求所经过的整个路径
    
- 轨迹包含跨度(Span)：服务之间的跨度、接口调用之间的跨度等。
    
- 跨度是轨迹的基石
    

指标(Metric)：

- 定义为在运行时捕获的服务的度量数据。
    
- 度量数据可以是系统调用的持续时间、CPU或内存使用情况。
    

日志(Logs)：

- 带时间戳的文本记录
    

上下文信息(Baggage)：

- OpenTelemetry可以跨多个服务之间的多个跨度传播上下文信息，比如用户ID。你可以将其附加到跟踪中的每个跨度。
    
- 使用上下文信息时有一些注意事项，上下文信息存储在HTTP头中，它将暴露给任何可以检查你的网络数据包的人。
    

OpenTelemetry的构成

语义约定：为常见的遥测类型(如应用程序名称、HTTP调用等)定义标准命名约定。

- API：定义要生成的遥测、度量、日志和跟踪。
    
- SDK：OpenTelmetry的一些sdk可以为通用库和框架提供自动检测。
    
- OpenTelmetry协议(OTLP)：用于向可观察性后端发送数据的协议。
    
- 跨服务传播器：传播器在不同的服务和进程之间传输数据。
    
- 资源探测器：将产生遥测的实体标记为资源属性。
    

OpenTelemetry数据处理

- 数据收集器：定义收集器如何摄取数据，推或拉。
    
- 数据处理器：定义在导出数据之前如何处理数据。
    
- 数据导出器：定义如何将数据发送到可观察性后端，推或拉。
    

## 在Rust中使用OpenTelemetry

在开始检测应用程序之前，我们需要向一个可观察性的后端来发送遥测数据，最容易上手的就是Jaeger。

设置Jaeger

启动Jaeger的最简单方法是使用Docker镜像：

启动Jaeger的最简单方法是使用Docker镜像：

```
docker run -d --name jaeger \  -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 \  -e COLLECTOR_OTLP_ENABLED=true \  -p 6831:6831/udp \  -p 6832:6832/udp \  -p 5778:5778 \  -p 16686:16686 \  -p 4317:4317 \  -p 4318:4318 \  -p 14250:14250 \  -p 14268:14268 \  -p 14269:14269 \  -p 9411:9411 \  jaegertracing/all-in-one:1.47
```

在这里16686是前端显示端口；4318是接收遥测数据的HTTP端口，4317是接收遥测数据的GRPC端口，它们都遵循OTLP协议。

一旦它启动并运行，可以在浏览器中输入：http://localhost:16686/ 来观察数据。

## 创建项目

在这个例子中，定义一个简单的函数生成一个随机数并打印出来，然后在执行时返回一个结果。

使用如下命令创建一个Rust项目：

```sh
cargo new otlp-example
```

剩下的就去看代码吧...

