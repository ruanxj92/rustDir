#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}
//允许数据直接附加到枚举的变体中
#[derive(Debug)]
enum Message {
    Quit,
    //#[warn(dead_code)]
    Move {x:i32, y:i32},
    Write (String),
    ChangeColor (i32,i32,i32),
}

impl Message {
    fn call(&self) {
        println!("call {:#?}", self);
        match self {
            Message::Move{x, y}=>{
                println!("x: {}, y: {}", x, y);
            }
            Message::Quit=>{} 
            Message::Write(_)=>{} 
            Message::ChangeColor(_, _, _) => {
            }
        }
    }
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("0"));

    route(four);
    route(six);
    //route(IpAddrKind::V6);

    let q = Message::Quit;
    let m = Message::Move {x:12, y:24};
    let w = Message::Write (String::from("Hello"));
    let c = Message::ChangeColor(0,255,255);

    q.call();
    m.call();
    w.call();
    c.call();

}

fn route(kind: IpAddrKind) {
    println!("route {:#?}", kind);
}