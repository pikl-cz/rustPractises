pub fn run (size_x: i32) {
    println!("Čtverec");

    let x: i32 = size_x;

    for _a in 0..x {
        for i in 0..x {
            print!("x");

            if i == x - 1 {
                println!("");
            }
        }

    }


}