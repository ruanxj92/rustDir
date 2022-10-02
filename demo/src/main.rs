#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    //方法
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    //关联函数，类似静态函数
    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width:30,
        length:50,
    };
    let rect2 = Rectangle{
        width:10,
        length:40,
    };
    let rect3 = Rectangle{
        width:35,
        length:55,
    };
    let sq1 = Rectangle::square(10);
    println!("{:#?}", sq1);
    println!("area {}", rect1.area());
    println!("{}",rect1.can_hold(&rect2));
    println!("{}",rect1.can_hold(&rect3));
    //println!("{}", rect);//`Rectangle` doesn't implement `std::fmt::Display`
    println!("{:?}", rect2);// `Rectangle` doesn't implement `Debug` , add #[derive(Debug)]
    println!("{:#?}", rect3);// `Rectangle` doesn't implement `Debug` , add #[derive(Debug)]
}
