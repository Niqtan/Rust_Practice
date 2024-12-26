fn main() {
    let s = String::from("hello");

    takes_ownership(s); 
    println!("{}", s);
    //Not allowed since the takes_ownership function already gets its value
    
    let x = 5;
    makes_copy(x);
    println!("{}", x);
    //Valid since by ownership rules, x only gets copied since its datatype
    // is an integer
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}