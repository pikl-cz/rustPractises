pub fn run (size_x: i32) {
    println!("Trojúhelník (min. délka základny se nastaví na 3)");

    let mut size: i32 = size_x;


    if size % 2 != 0 {
        size += 1;
    }

    let mut iterator: i32 = size;
    while iterator >= 0 {

        if iterator % 2 != 0 {
            let spaces: i32 = iterator / 2;
            for _s in 0..spaces {
                print!(" ");
            }

            for _a in iterator..size {
                print!("x");
            }
            println!("");
        }

        iterator -= 1;
    }

}