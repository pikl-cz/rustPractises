// Součet prvních n přirozených čísel.

use std::io;

fn main() {

    println!("Napiš číslo:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Selhalo načtení čísla!");

    println!("Tvoje číslo je: {number}")
}