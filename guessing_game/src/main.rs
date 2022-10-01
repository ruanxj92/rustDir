use std::io;
use rand::Rng;
fn main() {
    println!("guess a number ");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("secrt_number is {secret_number}");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("cannot read line");

    println!("you guess:{}", guess);
}
