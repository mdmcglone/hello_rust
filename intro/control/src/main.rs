fn main() {


    let mut fib0 = 0;
    let mut fib1 = 1;
    let mut _fib2 = 0;
    println!("{fib0}");
    println!("{fib1}");
    
    while fib1 < 1000 {
        _fib2 = fib0 + fib1;

        fib0=fib1;
        fib1=_fib2;

        if fib1 > 1000 {
            break
        }

        println!("{fib1}");

    }


    // let mut count = 0;

    // 'counting_up: loop {

    //     println!("count = {count}");

    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");

    //         if remaining == 1 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1
    //     }
    //     count += 1

    // }

    // println!("end count = {count}");


    // let list = [15,25,35,45,55];
    // for elem in list {
    //     println!("the value of list element is {elem}");
    // }





    // let number = 7;

    // let mut condition = true;

    // if number < 5 {
    //     println!("condition true");
    //     condition = true;
    // } else {
    //     println!("condition false");
    //     condition = false;
    // }

    // let num2 = if condition {2} else {4};
    // println!("conditional number is {num2}");

}
