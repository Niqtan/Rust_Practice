use std::io; //Uses a library called std::io which allows us to  use its function
use std::cmp::Ordering;
use rand::Rng;
// Pretty printing
use colored::*; 

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    // thread_rng allows us to use a random name generator
    // Gen range chooses a number between 1 and 101
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //Associated functon of the String type == Creates a new string

        io::stdin() // Give us a handle of the standard input (Rust's way of input())
            .read_line( &mut guess) //Gets the input from the user and then append it to the variable guess, which creates a new string
            //& is a type of references
            //Read line returns a result type which is an enumeration (an enumeration basically means that it can
            //be in multiple states, hence it is a variant.)
            .expect("Failed to skibidi toilet"); // Used to capture errors instead of sending compilation warnings

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, //Underscore is a catchall value which matches all Error values and catches these errors
            };
            //Methods: Trim will remove any whitespace and parse will parse the string INTO an unsigned integer
            // Except will handle errors within the input

            //The reason why guess can be defined again is because of shadowing
            // Shadowing allows us to reuse the guess variabes rather than having two unique variables

            println!("You Guessed {}", guess);

            match guess.cmp(&secret_number) { //match expression  
                Ordering::Less => println!("{}", "Too small!".red()), // First arm
                Ordering::Greater => println!("{}", "Too large!".red()), //Second Arm
                Ordering::Equal => {
                println!("{}", "You win".green());
                break; } //Third arm
            } //Compares two things via the ordering enum which allows us to use the
            //variants less, greater, and equal
    }
}
