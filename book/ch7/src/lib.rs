mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//Specify a path
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    
}

//Example 1
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {

}
fn function2() -> IoResult<()> {

}


//Nested Path
use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::{self, Write};
use std::io::*;
//Glob operator

//Re Exporting
mod front_of_house;
//Tells rust to find the module with the same file anme


//Specify a path
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    
}