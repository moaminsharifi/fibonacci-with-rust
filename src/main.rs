use std::io;

fn fibonacci_calculate(number: u128) -> u128 {
    if number == 1 || number == 0 {
        return 1;
    }
    let mut count: u128 = 0;
    let mut num_1: u128 = 0;
    let mut num_2: u128 = 1;
    while count < number {
        let holder = num_1 + num_2;
        num_1 = num_2;
        num_2 = holder;
        count = count + 1;
    }
    num_1
}
fn main() {
    println!("Welcome to Fibonacci Calculator with rust");
    println!("Enter Number to Calculate");
    let mut number_io = String::new();
    io::stdin()
        .read_line(&mut number_io)
        .expect("Failed to read line");
    let number: u128 = number_io.trim().parse().unwrap();
    let fibonacci: u128 = fibonacci_calculate(number);
    println!("You entered {number} and result is {fibonacci}");
}
