use std::io;

fn main() {

    loop {
        println!("\ninput a temperature below in the format like 30c or 90 F:");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        input = input.trim().to_string();

        let (temp, units) = format(&input);

        if temp == "Error" {
            println!("cannot parse input. Try again!");
            continue
        }

        // parse temp to f32
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("cannot parse temperature. Try again!");
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


}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0

}

fn c_to_f(temp: f32) -> f32 {
    (9.0 * temp / 5.0) + 32.0

}



fn format(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'c' || item == b'C' || item == b'f' || item == b'F' {
            return (&s[..i], &s[i..]);
        }
    }

    // else, return error
    return ("Error", "units not found");

}            

