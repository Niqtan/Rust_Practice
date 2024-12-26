//Special implement methods

//Conditionally implement methods
//This section gives a brief overview of how traits can also be
// used as a condition in order to implement something
use std::fmt::Display;


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair <T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display +PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            prinln!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {

}

//Blanket implementations

impl<T: Display> ToString for T {
    //Snip
}


