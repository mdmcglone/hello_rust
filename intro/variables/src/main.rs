fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("the value of x in the inner scope is {x}");

    }

    println!("the value of x is {x}");


    let tup = (500, 69, 1);

    let (_x, _y, z) = tup;

    println!("the value of z is {z}");

    let fivehundred = tup.0;

    let sixtynine = tup.1;

    let one = tup.2;

    let nums: [u32; 4] = [1,2,3,4];

    let threes: [3; 5];

    let onethree = threes[1];

}
