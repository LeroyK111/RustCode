# 云服务构建

## Rust微服务Rust-RPC框架
krpc: https://github.com/kwsc98/krpc-rust

实现一个基于netty单路复用网络模型的rpc框架，支持spring-boot启动，支持zookeeper，nacos注册中心。


## RustDesk 远程桌面

RustDesk -- 基于 Rust 的开源远程桌面
RustDesk 是一个基于 Rust 开发的开源远程桌面，TeamViewer 的替代品。RustDesk 开箱即用，无需任何配置。您完全掌控数据，不用担心安全问题。您可以使用我们的注册/中继服务器， 或者自己设置， 亦或者开发您的版本。




## 分布式存储

### helyim
https://github.com/helyim/helyim

```
helyim 是使用 rust 重写的 seaweedfs，具体架构可以参考 Facebook 发表的 haystack 和 f4 论文。

主要设计目标为：
精简文件元数据信息，去掉对象存储不需要的 POSIX 语义（如文件权限）

小文件合并成大文件，从而减小元数据数，使其完全存在内存中，以省去获取文件元数据的磁盘IO

支持地域容灾，包括 IDC 容灾和机架容灾

架构简单，易于实现和运维

支持的特性：
支持使用 Http 的文件上传，下载，删除，查看集群状态等接口

Volume 数据支持 Replica Placement

基于 Openraft 的元数据服务的 Failover 功能

冗余数据的定时删除

元数据接口的请求重定向

未来可能实现的功能：
支持纠删码（高优先级）

支持 Filer 服务（高优先级，以支持 S3，HDFS，Fuse 等 proxy）

支持 io-uring

支持 kernel bypass（RDMA，DPDK，SPDK）

GPU direct storage（大概率不做，但是会学习相关知识）
```

```sh
cargo run --bin helyim master
```

```sh
cargo run --bin helyim volume --port 8080 --folders ./data:7
```


```sh
# 获取文件 ID
> curl http://localhost:9333/dir/assign

{"fid":"6,16b7578a5","url":"127.0.0.1:8080","public_url":"127.0.0.1:8080","count":1,"error":""}

# 上传文件
> curl -F file=@./sun.jpg http://127.0.0.1:8080/6,16b7578a5

{"name":"sun.jpg","size":1675569,"error":""}
```

```sh
curl -X DELETE http://127.0.0.1:8080/6,16b7578a5
```