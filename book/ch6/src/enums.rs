enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String)
}

enum Message {
    Quit, //unit struct
    Move {x: i32, y: i32}, //anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn some_function(&self) {
        println!("LGR");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {

}