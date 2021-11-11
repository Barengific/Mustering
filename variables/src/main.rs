fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of maxpint is: {}", MAX_POINTS);

    const MX_POINTS: u32 = 10000_000;
    println!("The value of maxpint is: {}", MX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let d: String;

    d = "bob".to_string();
    println!("The value of ddd is: {}", d);

    let spaces = " ";
    let spaces = spaces.len();

    let arr: (i32, f64, u8) = (500, 6.4, 1);

    loop{
        let mut i: u8 = 0;
        println!("******The value of ddd is: {}", arr.1);
        i = i + 1;
        break;
    }
}
