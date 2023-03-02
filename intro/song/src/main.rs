use std::io;

fn main() {

    loop {
        println!("\ninput a temperature below in the format like 30c or 90 F:");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        // let input = "45c";

        let reversed: String = input.chars().rev().collect();

        let (mut units, temp_rev) = reversed.split_at(2);

        units = units.trim();

        let temp_rerev: String = temp_rev.chars().rev().collect();

        let temp: f32 = match temp_rerev.trim().parse() {
            Ok(flt) => flt,
            Err(_) => {
                println!("not a number. Try again!");
                continue
            }
        };
        
        
        if units.contains("f") || units.contains("F") {
            let conversion = f_to_c(temp);
            println!("\n{temp} degrees {units} is equal to {conversion} degrees c");
            continue
        } else if units.contains("c") || units.contains("C") {
            let conversion = c_to_f(temp);
            println!("\n{temp} degrees {units} is equal to {conversion} degrees f");
            continue
        } else {
            println!("cannot tell temperature units. Try again!");
            continue
        }
    }

    // let numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    // let events = ["a partridge in a pear tree", "two turtle doves", "three french hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    // let mut count = 0;
    // while count < 12 {
        
    //     for i in (0..=count).rev() {
    //         println!("on the {} day of christmas my true love gave to me, {}", numbers[i], events[i]);
    //     }
    
    // println!("\n");

    // count += 1

    // }

}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0

}

fn c_to_f(temp: f32) -> f32 {
    (9.0 * temp / 5.0) + 32.0

}