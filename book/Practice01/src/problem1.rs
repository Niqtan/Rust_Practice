use std::io;

fn main() {
    println!("Let's Get RUSTY!");
    println!("Which conversion would you like to do?");
    println!("1. Celcius to Fahrenheit");
    println!("2. Fahrenheit to Celcius");

    let mut choice = String::new();

    loop {
    io::stdin() 
        .read_line(&mut choice).expect("Failed to read your input");
    //Trim function is necessary to cut off white space
    //This is because the read_line function gives a new line while reading the user input
    if choice.trim() == "1" { 
        break celcius();
    }
    else if choice.trim() == "2" {
        break fahrenheit();
    }
    else {
        println!("Not a valid choice!");
        continue
    }
};
}

fn celcius() {
    println!("Please Input Celcius to Fahrenheit:");

    let mut celcius = String::new();

    io::stdin() 
        .read_line(&mut celcius).expect("Failed to read your input");

    let celcius: f32 = celcius.trim().parse().expect("Not an integer bruh");

    let fahrenheit = (celcius * 9.0/5.0) + 32.0;
    println!("You entered {celcius} so you get {fahrenheit}.");
}

fn fahrenheit() {
    println!("Please Input Fahrenheit to Celcius:");

    let mut fahrenheit = String::new();

    io::stdin() 
        .read_line(&mut fahrenheit).expect("Failed to read your input");

    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Not an integer bruh");

    let celcius = (fahrenheit - 32.0) * 5.0/9.0;
    println!("You entered {fahrenheit} so you get {celcius}.");
}