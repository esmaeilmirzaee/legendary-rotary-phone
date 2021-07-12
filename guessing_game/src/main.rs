use std::io;
use rand::Rng;
use std::cmp::Ordering; 

fn main() {
    println!("Welcome to guessing game!");
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess=String::new();
    
    loop {
        println!("Please input your guess.");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
        
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less=>println!("Too small"),
            Ordering::Greater=>println!("Too great"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}
