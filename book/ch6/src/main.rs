//Match Control Flow for normal enums

fn normal() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
//Debug Trait for pretty printing
#[derive(Debug)]
enum UsState {
    Alabama, 
    Alaska,
    NewYork,
    California
}
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    }
    }
}
    

fn match(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None
        //Says if its anything besides the match above with x, then return
        //None
    }
}

//If let syntax

fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Three"),
        _ => (),
    }
    //If let syntax

    if let Some(3) = some_value {
        println!("Three!");
    }
}