fn main() {
    let x = 5; //Concept of m5utability 
    println!("{x}");
    let x = 6 ;
    println!("{x}");
    //Concept of shadowing 

    //Data types in Rust

    //Scalar types 
    let flow: u8 = 25;

    //FLoating point types
    let f: f64 = 2.0;

    // Boolean Types
    let bollean: bool = true;

    // Char types
    let rusty: char = 'z';

    //Compound Types

    let tup: (&str, i32) = ("LGR", 100_1000)
    let ("Channel Name", "Sub count") = tup; 
    // Dot notation
    let sub_count = tup.1 //Always start at the index of 0

    //Arrays
    let 8_ball: i32 = [1_bol, 2_bol, 3_bol]; //Arrays typically are immutable by nature
    //Use vectors if you would want to dynamically expand it

    let resounding_sounds = ["40GHz", 5]; 
}