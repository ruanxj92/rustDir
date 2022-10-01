use std::io;

fn main() {
    println!("guess a number ");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("cannot read line");

    println!("you guess:{}", guess);
}
