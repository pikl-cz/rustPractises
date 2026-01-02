
// 1. Součet prvních n přirozených čísel.

use std::io;

pub fn run() {

    println!("Napiš číslo:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Selhalo načtení čísla!");

    let number: i32 = input.trim().parse().expect("Zadejte platné číslo!");

    let result: i32 = soucetPrvnichNPrirozenychCisel(number);


    println!("Součet prvních n přirozených čísel je: {result}");
}

fn soucetPrvnichNPrirozenychCisel(n: i32) -> i32 {
    n * (n + 1) / 2
}