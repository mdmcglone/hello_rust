fn main() {
    // let user1 = User {
    //     active: true, 
    //     username: String::from("m"), 
    //     email: String::from("m@m"), 
    //     sign_in_count: 10
    // };

    // println!("{}", user1.username);

    // let mut user2 = User {
    //     email: String::from("emmy@m"),
    //     ..user1
    // };

    // user2.username = String::from("emmy");

    // println!("{}", user2.username);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }    
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if rect.height < self.height && rect.width < self.width {
            true
        } else {
            false
        }
    }
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1
//     }

// }


// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

