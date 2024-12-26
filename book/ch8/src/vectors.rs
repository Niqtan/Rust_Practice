// Creating and putting values to a vector

fn main() {
    let a = [1, 2, 3];


    let mut v: Vec<i32> = Vec::new();
    v.push(1);

    let v2 = vec![1, 2, 3];
}

//Accessing elements in a vector
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("{third}");

    match v.get(2) {
        Some(third) => println!("{third}"),
        None => println!("No third element"),
    }
}

//Iterating over a vector
fn main() {
    let mut v = vec![1, 2, 3, 4 ,5];

    for i in &mut v {
        *i += 50;
    }
    
    for i in &mut v {
        println!("{i}")
    }
 }

 
 //Using an enum to store multiple types

 fn main(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{i}"),
        _ => println!("Skibidi Toilet")
    }
 }