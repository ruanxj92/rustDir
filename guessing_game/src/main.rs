use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);// i32
    println!("secrt_number is {secret_number}");
    let mut guess = String::new();
    loop {
        println!("guess a number ");
        io::stdin()
        .read_line(&mut guess)
        .expect("cannot read line");
        
        println!("you guess:{}", guess);
        //shadow same name
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => {
                println!("match num");
                num
            }
            Err(_) => {
                println!("match error");
                continue
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
