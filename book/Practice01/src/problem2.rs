use std::io;

fn main() {
    println!("Which Fibonacci number would you like to know about?");

    let mut fib_choice = String::new();
    io::stdin()
        .read_line(&mut fib_choice).expect("Not a valid fibonacci number.");

    let fib_choice: u32 = fib_choice.trim().parse().expect("Please enter a valid fib number!");

    let series = (fib_choice - 1) + (fib_choice - 2);
    println!("{fib_choice} is {series} of the Fibonacci Series");
}