use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);// i32
    // println!("secrt_number is {secret_number}");

    loop {
        println!("guess a number ");
        let mut guess = String::new();
        match io::stdin()
        .read_line(&mut guess){
            Ok(size) => {
                println!("read length: {}", size);
                println!("read: {}", guess);
            }
            Err(what_error)=> {
                println!("read stdin error: {}", what_error.to_string());
                return ;
            }
        };
        
        println!("you guess:{}", guess);
        //shadow same name
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => {
                println!("match num");
                num
            },
            Err(parse_error) => {
                println!("match error: {}", parse_error.to_string());
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
            }
            Ordering::Greater => {
                println!("too big");
            }
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
