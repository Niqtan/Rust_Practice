fn main() {
    //Rule 3 Prime example
    //s not yet valid....
    let s = "hello"
    //s is now valid! You may use it now

    //Dynamic and mutable now, and hence, stored in the heap
    let mut s = String::from("hello");
    //In languages such as C++, you would need to use functions for it to be stored
    //In rust, it automatically stores and deletes
    //Once s is no longer valid or out of scope, Rust deletes it 
}
//s is not recognized here anymore

//How variables and data interact
fn memory() {
    let x = 5;
    let y = x;
    //Copy trait

    let s1 = String::from("Hello");
    let s2 = s1.clone();  //Move (Not shadowing)
    // Not possible since Rust ownership rules dictate that only one
    // owner can only have ONE value
    //So if s1 has a value and s2 takes it, then s2 gets the value from s1


    println!("{}, world!", s1);
    //This will result in a compilation error since s1 does not exist anymore 
}
