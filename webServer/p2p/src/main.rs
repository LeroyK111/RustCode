/*
! websocket
*/



usestd::net::ToSocketAddrs;

useclap::Parser;

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args{
    ///要连接的客户端地址列表
    #[arg(short,long,value_delimiter=',',value_parser=parse_peer)]
    peers:Vec<String>,

    ///绑定服务器的地址
    #[arg(short,long,value_parser=parse_bind)]
    bind:String,
}

///解析并验证客户端url
fn parse_peer(s:&str)->Result<String,String>{
    //验证以ws://或wss://开头的URL
    if s.starts_with("ws://"){
        letip_port=&s[5..];
        ifletOk(_socket_addr)=ip_port.to_socket_addrs(){
            returnOk(s.to_string());
    }
    }
    Err(format!("InvalidclientURL:{}",s))
}

///解析并验证绑定地址
fn parse_bind(s:&str)->Result<String,String>{
    if let Ok(_socket_addr)=s.to_socket_addrs(){
        returnOk(s.to_string());
    }
    Err(format!("Invalidbindaddress:{}",s))
}