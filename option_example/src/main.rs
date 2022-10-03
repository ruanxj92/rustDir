fn main() {
    println!("Hello, world!");
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
    if_let_example();
}
fn if_let_example(){
    let v = Some(3u8);

    match v{
        Some(3) => {
            println!("three");
        }
        _=>{
            ()
        }
    }
    //if let 处理单分支情况, 与上面的match等价
    if let Some(3) = v{
        println!("three");
    }
}