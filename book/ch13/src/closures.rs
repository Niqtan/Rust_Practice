
//Example 01
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

//Closure Type inference and Annotation

// Closures dont do this
fn some_func(parm1: u32) -> //sum return value { value }

let expensive_closure = |num: u32| /* argument of closure */ -> u32 {
    println!("Calc is short for calculator");
    thread::sleep(Duration::from_secs(2));
    num
}

//Differentiation of functions and closures
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };

let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ; //Brackets are optional for closures

//Closure definitions
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

//Listing 13-4
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); //The functionality
    //definec

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
//Mut borrow
let mut list = vec![1, 2, 3];
println!("Before defining closure: {list:?}");

let mut borrows_mut = || list.push(7); //The functionality
//defined
borrows_mut();
println!("After closure: {list:?}");

//Closure taking ownership
use std::thread;
 
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    //Move keyword for taking ownership
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

//Implementation of the unwrap_or_else method on Option<T>
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

//Example 02 of a method implementation
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

//Defined to takenthe trait FnMut because it calls the closure multiple
//times once for each item in the slice

//Lifetimes with closures
fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}