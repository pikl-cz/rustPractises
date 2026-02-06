use crate::gui_headline;

pub fn run(task: &str) {
    gui_headline::run();
    println!("TiTr - Pause úkolu '{}'", task);
    println!("--------------");
    println!("Funkce pause ještě není implementována.");
}

