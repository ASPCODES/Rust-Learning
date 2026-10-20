use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
     println!("The secret number is: {secret_number}");

    println!("Please input your Guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You Guessed: {guess}");

    let x = 5;
    let y = 10;

    print!("x = {x} and y + 2 = {}", y + 2);
}
