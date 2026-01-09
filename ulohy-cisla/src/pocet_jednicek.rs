use std::io;

pub fn run() {
    println!("Napiš číslo:");

    let mut input_number = String::new();

    io::stdin()
    .read_line(&mut input_number)
    .expect("Selhalo načtení čísla!");

    // let number: i32 = inputNumber.trim().parse().expect("Zadejte platné číslo!");

    let result: i32 = pocet_jednicek(input_number);

    println!("Počet jedniček v čísle je: {}", result);
}

fn pocet_jednicek(n: String) -> i32 {
    // Řešení přes iterátory
    // // https://doc.rust-lang.org/std/primitive.str.html#method.trim
    // n.trim()
    // // https://doc.rust-lang.org/std/primitive.str.html#method.chars
    // .chars()
    // // https://doc.rust-lang.org/std/primitive.array.html#method.map
    // .map(|c| c.to_digit(10).unwrap() as i32)
    // // https://doc.rust-lang.org/std/iter/trait.Sum.html#tymethod.sum
    // .sum()

    // Řešení přes for loop
    let array_of_numbers: Vec<char> = n.trim().chars().collect();
    let mut iterator = 0;

    println!("Počet čísel: {}", array_of_numbers.len());

    for x in 0..array_of_numbers.len() {
        let actual_number:u32  = array_of_numbers[x].to_digit(10).unwrap();
        if actual_number == 1 {
            iterator += 1
        }
    }

    iterator
}