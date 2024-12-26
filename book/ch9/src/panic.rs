/*

fn main() {
    panic!("Bruh");
}
*/
//using Panic!
fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Bawal yan boy");
    }
}