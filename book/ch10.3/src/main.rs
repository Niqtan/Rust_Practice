struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl <'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attention please {}", announcement);
        self.part
    }

}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

//Lifetime Elision
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//Static Lifetimes
fn main() {
    let s: &'static str = "I have a static lifetime.";

}

//Grand Finale of Chapter 10
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}