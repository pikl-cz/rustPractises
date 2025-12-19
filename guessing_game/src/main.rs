// poslední čtené: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number

use std::io;

fn main() {
    println!("Gues the number!");

    println!("Please input your guess.");

    // let guess = String::new(); // immutable - neměnné (by default)
    let mut guess = String::new(); // mutable - měnitelné

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}