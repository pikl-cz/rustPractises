use std::io;

pub fn run() {
    println!("Napiš číslo:");

    let mut input_number = String::new();

    io::stdin()
    .read_line(&mut input_number)
    .expect("Načtení selhalo");

    let n: i32 = input_number.trim().parse::<i32>().expect("Zadej platné číslo");
    let result: i32 = odmocnina_z_n(n);
    let resquared: i32 = result * result;

    // Podobné jako s expect (akorát to nemá vlastní hlášku)
    // let result: i32 = odmocnina_z_n(input_number.trim().parse::<i32>().unwrap());
    println!("Zadané číslo je: {n},\ncelá část odmocniny z čísla n je: {result}.\nN na druhou je {resquared}.");
    println!("-----------");
}

fn odmocnina_z_n(n: i32) -> i32 {
    let mut res = 0;
    let mut i = 0;

    while i <= n {
        if (i * i) <= n {
            res = i;
        }
        i += 1;
    }

    res
}