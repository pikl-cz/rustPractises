
// 1. Součet prvních n přirozených čísel.

use std::{io, io::Write};

pub fn run() {

    print!("Napiš číslo, u kterého se má spočítat faktoriál: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Selhalo načtení čísla!");

    let number: i32 = input.trim().parse().expect("Zadejte platné číslo!");

    // println!("\n");

    // let result: i32 = faktorial_via_while(number);
    let result: i32 = faktorial_via_for(number);


    println!("Faktoriál čísla je: {result}");
}

fn faktorial_via_for(n: i32) -> i32 {

    let mut number: i32 = 1;
    for x in 1..n + 1 {
        number = x * number;
        // println!("{x}");
    }

    number
}

// fn faktorial_via_while(n: i32) -> i32 {

//     let mut number: i32 = n;
//     let mut result: i32 = 1;
//     while number > 0 {
//         result = result * number;
//         println!("{number}");
//         number -= 1;
//     }

//     result
// }