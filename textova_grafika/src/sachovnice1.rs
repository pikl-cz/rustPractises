pub fn run (x: i32) {

    let mut firstChar: char = '#';

    for a in 0..x {

        if a % 2 == 0 {
            firstChar = '#';
        } else {
            firstChar = '.';
        }

        for b in 0..x {
            if firstChar == '#' && b % 2 == 0 {
                print!(".");
            } else if firstChar == '.' && b % 2 == 0 {
                print!("#");
            } else {
                print!("{firstChar}");
            }

            if b == x {
                println!();
            }
        }


        // if a % 2 == 0 {
        //     for b in 0..x {
        //         if b % 2 == 0 {
        //             print!(".");
        //         } else {
        //             print!("#");
        //         }

        //         if b == x {
        //             println!();
        //         }
        //     }
        // } else {
        //     for b in 0..x {
        //         if b % 2 == 0 {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }

        //         if b == x {
        //             println!();
        //         }
        //     }
        // }

        println!("");
    }

}