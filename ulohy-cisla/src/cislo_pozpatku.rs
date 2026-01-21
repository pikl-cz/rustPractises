use std::io;
// use std::cmp::Reverse;

pub fn run() {
    println!("Napiš číslo:");

    let mut input_number = String::new();

    io::stdin()
    .read_line(&mut input_number)
    .expect("Selhalo načtení čísla!");

    let result_fce: i32 = cislo_pozpatku_pomoci_fci(input_number.clone());
    let result_while: i32 = cislo_pozpatku_while(input_number.clone());
    let result_for: i32 = cislo_pozpatku_for(input_number.clone());
    println!("Číslo pozpátku pomocí fcí je: {}", result_fce);
    println!("Číslo pozpátku pomocí while je: {}", result_while);
    println!("Číslo pozpátku pomocí for je: {}", result_for);
}

fn cislo_pozpatku_while(n: String) -> i32 {

    // Udělat pole znaků z čísla
    let array_of_numbers: Vec<char> = n.trim().chars().collect();

    // Počet znaků v poli
    let chars_length: usize = array_of_numbers.len();
    // println!("pocet čísel {:?}", chars_length);

    let mut final_number = String::new();

    // Pomocí while loop
    let mut iterator = chars_length;
    while iterator > 0 {
        // println!("{}", iterator);
        // print!("{:?}", array_of_numbers[iterator - 1]);

        final_number.push(array_of_numbers[iterator - 1]);
        iterator -= 1;
    }


    final_number.trim().parse::<i32>().unwrap()
}

fn cislo_pozpatku_for(n: String) -> i32 {
    // Udělat pole znaků z čísla
    let array_of_numbers: Vec<char> = n.trim().chars().collect();

    // Počet znaků v poli
    let chars_length: usize = array_of_numbers.len();
    // println!("pocet čísel {:?}", chars_length);

    let mut final_number = String::new();

    // Pomocí for loop
    for x in (0..chars_length).rev() {
        final_number.push(array_of_numbers[x]);
    }


    final_number.trim().parse::<i32>().unwrap()
}

fn cislo_pozpatku_pomoci_fci(n: String) -> i32 {
    let array_of_numbers: Vec<char> = n.trim().chars().collect();
    array_of_numbers.iter().rev().collect::<String>().trim().parse::<i32>().unwrap()
}