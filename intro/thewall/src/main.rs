use std::io::{self, BufRead};

fn main() {
    // take keyboard inputs like up and down
    // and move the player accordingly

    let thewall = "|||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||";

    let mut ups = 0;
    let mut downs = 0;

    const SIZE: i32 = 25;

    println!("Press w to move up, s to move down \n");
    println!("Observe the wall!\n");
    println!("{thewall}");
    

    loop {

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
    
        if input.trim() == "w"
        {
            ups += 1;
        }
        else if input.trim() == "s"
        {
            downs += 1;
        }
        else {
            println!("Invalid input");
        }

        let mut pos = ups - downs;

        // println!("Ups: {}, Downs: {}, Pos: {}", ups, downs, pos);

        if pos < 1 {
            pos = 0;
            ups = 0;
            downs = 0;
        } 
        if pos > SIZE {
            pos = SIZE;
            ups = SIZE;
            downs = 0;
        }



        let nones = SIZE - pos;

        for _i in 0..=nones/2 {
            println!("\n");
        }
        for _i in 0..=pos {
            println!("{thewall}\n");
        }
        for _i in 0..=nones/2 {
            println!("\n");
        }
        println!{"\n"}




    }

}
