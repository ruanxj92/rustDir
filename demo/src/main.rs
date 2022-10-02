#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}

fn main() {
    let rect = Rectangle{
        width:30,
        length:50,
    };

    println!("area {}", area(&rect));
    //println!("{}", rect);//`Rectangle` doesn't implement `std::fmt::Display`
    println!("{:?}", rect);// `Rectangle` doesn't implement `Debug` , add #[derive(Debug)]
    println!("{:#?}", rect);// `Rectangle` doesn't implement `Debug` , add #[derive(Debug)]
}
