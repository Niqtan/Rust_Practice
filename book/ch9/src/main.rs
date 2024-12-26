//Recoverable errors

//Recall a result type in Rust has two return values: Ok or ERR
/* 
fn main() {
    enum Resutlt<T,E> {
        Ok(T),
        Err(E),
    
    
    }
}
*/

//Example
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("{e}")
            },
            other_error => {
                panic!("Not {other_error}");
            }
        }
    };
}

//Example 1 alternative
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("Hello.txt").unwrap();

    //Expect method
    let f = File::open("Hello.txt").expect("Failed")

    /* 
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };
    */
}

use std::fs::{self, File};
use std::io::{self, Read};

//Simplest verison
fn read_username_from_file() -> Result<String, io::Error> {
    //Opening hello.txt may fail here so we get a result type 
    fs::read_to_string("hello.txt")

    //Read the string of our file and append that string
}

//Cannot use ? operator here because the main function does not return
// anything.
fn main() -> Result<(), Box<dyn Error>>  {
    let f = File::open("hello.txt")?;

    Ok(())
}


