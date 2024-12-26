fn something() {
     let mut s1 = String::from("hello");
     let (len) = calculate_length(&mut s1);

     println!("The length of {s1} is {len}.");
}

fn calculate_length(s: mut &String) -> usize {
    let length = s.len();
    length
}

//Mutable reference restriction
fn skibidi() {
    let mut r1 = String::from("Hello");

    let r2 = &mut r1;


    println!("{r2}  ");

    let r4 = &mut r1;
    println!("{r4}");
}

// dangling

fn main() {
    let refer_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s 
    //Deallocated once the function ends
}