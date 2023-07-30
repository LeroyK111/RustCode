/*
枚举结构体
*/
enum IpAddrKind {
    V4,
    V6,
}

pub fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    
}
