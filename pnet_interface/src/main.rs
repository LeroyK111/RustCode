use pnet::{
    datalink::{self, Channel},
    packet::ethernet::EthernetPacket,
};

/*
探索如何使用Rust中的pnet库查找和列出计算机上的网络接口。

当你从事底层网络编程任务(如数据包捕获或自定义数据包创建)时，了解网络接口是至关重要的。
*/

fn main() {
    /*
    网络接口
    */
    println!("Available interfaces:");
    let interfaces = datalink::interfaces();
    for (index, interface) in interfaces.iter().enumerate() {
        println!("{}: {}", index + 1, interface.name);
    }
    capturePackage()
}

fn capturePackage() {
    /*
    如何抓取数据包
    */
    // 选取 'en0' 网络接口
    let interface = datalink::interfaces()
        .into_iter()
        .find(|interface| interface.name == "en0")
        .unwrap();

    // 抓包需要数据链路通道，它建立了到网络接口'en0' 的底层链路。
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("开始读取网络数据包: ");

    // 建立一个循环来连续读取传入的数据包
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("新数据包:");
                    println!(
                        "{} => {}: {}",
                        ethernet_packet.get_destination(),
                        ethernet_packet.get_source(),
                        ethernet_packet.get_ethertype()
                    );
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
