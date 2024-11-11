# 使用pcap读取PCAP文件


pcap库允许你读取从网络捕获的文件，通常称为PCAP(数据包捕获)，其中包含网络流量的跟踪，此步骤对于分析网络事件或调试至关重要。

```rust
use pcap::Capture;

fn main() {
   let mut cap = Capture::from_file("example.pcap").unwrap();
   while let Ok(packet) = cap.next() {
       println!("Packet : {:?}", packet);
   }
}
```