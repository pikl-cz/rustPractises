use std::io;

pub fn run() {
    println!("Napiš číslo:");

    let mut input_number = String::new();

    io::stdin()
    .read_line(&mut input_number)
    .expect("Selhalo načtení čísla!");

    // let number: i32 = inputNumber.trim().parse().expect("Zadejte platné číslo!");

    let result: i32 = ciferny_soucet(input_number);

    println!("Ciferný součet čísla je: {}", result);
}

fn ciferny_soucet(n: String) -> i32 {
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
    let mut result = 0;

    println!("Počet čísel: {}", array_of_numbers.len());

    for x in 0..array_of_numbers.len() {
        println!("Číslo {} je: {}", x, array_of_numbers[x].to_digit(10).unwrap() as i32);
        result += array_of_numbers[x].to_digit(10).unwrap() as i32;
    }

    result
}