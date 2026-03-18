use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main() {
    println!("gussing number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed : {guess}");

    match guess.cmp(&secret_number){
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("You win"),
    }
}