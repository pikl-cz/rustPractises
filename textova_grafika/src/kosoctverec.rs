pub fn run (size_x: i32) {
    println!("Trojúhelník (min. délka základny se nastaví na 3)");

    // Horní polovina
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

    // Spodní polovina
    let mut iterator_down = size - 2;
    while iterator_down >= 0 {
        if iterator_down % 2 != 0 {
            // let spaces_down: i32 = iterator_down / 2;
            // for _s in 0..spaces_down {
            //     print!(" ");
            // }

            let mut spaces_down = size - iterator_down;
            spaces_down = spaces_down / 2;
            for _ss in 0..spaces_down {
                print!(" ");
            }

            for _i in 0..iterator_down {
                print!("x");
            }
            println!();
        }
        iterator_down -= 1;
    }


}