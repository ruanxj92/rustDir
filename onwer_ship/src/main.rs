//内存所有权给进去了
fn take_onwership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number:i32){
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

//reference and borrow
//把引用作为函数参数的行为叫做借用
fn calculate_length(some_string:&String) -> usize {
    some_string.len()
}
//一个区域内只能同时有一个可变引用
fn modify_string(s:&mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn main() {
    let s= String::from("Hello World");
    //默认是移动所有权
    take_onwership(s);
    let x=5;
    makes_copy(x);
    println!("x: {}", x);

    let s1 = gives_ownership();
    let _s2 = String::from("hello");
    let _s3 = takes_and_gives_back(_s2);
    let len = calculate_length(&s1);
    println!("length of {} is {}", s1, len);

    let mut s4 = String::from("hello");
    let l4 = modify_string(&mut s4);
    println!("length of {} is {}", s4, l4);
}
