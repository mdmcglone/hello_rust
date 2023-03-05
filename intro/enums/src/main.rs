fn main() {

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_str = Some("5");

    let absent_number: Option<i32> = None;

    let alaska_q = Coin::Quarter(USstate::Alaska);

    value_in_cents(&alaska_q);

    let five = Some(5);
    let six = plusone(five);
    println!("{:?}", six);
    let no = plusone(None);
    println!("{:?}", no);

    let confmax = Some(4);
    if let Some(max) = confmax {
        println!("confmax is {:?}", confmax);
    }

    let mut count = 0;

    if let Coin::Quarter(state) = alaska_q {
        println!("state is {:?}", state);
    } else {
        count += 1;
    }

}

fn plusone(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => None,
        Option::Some(i) => Some(i+1), 
    }

}

#[derive(Debug)]
enum USstate {
    Alabama, 
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USstate),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State {:?}", state);
            25
        }
    }
}


enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {



    }
}