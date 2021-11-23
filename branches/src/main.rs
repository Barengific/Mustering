fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false {}", false_finder());
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    println!(" ");
    println!("The value of number is: {}", main_q());
}

fn false_finder() -> i32 {
    10
}

fn main_q() -> i32 {
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    number
}
