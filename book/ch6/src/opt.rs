
fn opt() {
    //Option Enum

    enum Option<T> {
        Some(T), //Stores a value
        None,
    }

    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<i32> = None; 
    //Rust cannot understand the value being passed in, hence we would
    // have to annotate it
}


//Example #2
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = None;
    //Writes codee that handles all other variants
    let sum = x + y.unwrap_or(0);
}