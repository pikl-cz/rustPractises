use std::io;

mod ctverec;
mod trojuhelnik;
mod kosoctverec;

fn main() {
    let menu: Vec<(&str, &str)> = vec![
        ("čtverec", "ctverec"),
        ("trojúhelník", "trojuhelnik"),
        ("kosočtverec", "kosoctverec"),
        ("šachovnice1", "sachovnice1"),
        ("šachovnice2", "sachovnice2"),
        ("čtverce", "ctverce"),
        ("kruh", "kruh"),
        ("spirála", "spirala"),
        ("sinusovka", "sinusovka"),
    ];

    for (index, (popis, _klic)) in menu.iter().enumerate() {
        println!("{}: {}", index + 1, popis);
    }

    let mut menu_input = String::new();
    println!("\nZadej číslo obrazce:");

    io::stdin()
    .read_line(&mut menu_input)
    .expect("Selhalo načtení z menu");

    let mut size_x = String::new();
    println!("\nZadej šířku obrazce:");

    io::stdin()
    .read_line(&mut size_x)
    .expect("Selhalo načtení šířky obrazce");

    let size_x = size_x.trim().parse::<i32>().unwrap_or(0);

    menu_input = menu_input.trim().to_string();

    let idx = menu_input.parse::<usize>().unwrap_or(0);
    if idx > 0 && idx <= menu.len() {
        match menu[idx - 1].1 {
            "ctverec" => ctverec::run(size_x),
            "trojuhelnik" => trojuhelnik::run(size_x),
            "kosoctverec" => kosoctverec::run(size_x),
            // zde můžeš přidat další match větve pro další klíče
            _ => println!("\nTato funkce není implementována."),
        }
    } else {
        println!("\nKonec.");
    }
}
