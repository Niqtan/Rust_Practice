use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::new(); //Creates a new string
    let s2 = "hello world"; //string literal
    let s3 = s2.to_string(); //Converts string literal to string
    let s4 = String::from("initial contents"); //New string with content
}

//Appending a string
fn main() {
    let mut s = String::from("foo");
    let s2 = s.push_str("bar");

    //Concat
    let s1 = String::from("Love");
    let s2 = String::from("Diva");
    let s3: String = s1 + s2;

    //Format function
    let s3 = format!("{s1}{s2}")

}

//Indexing into Strings

fn main() {
    let hello: String = String::from("hello");

    for b in hello.graphemes(true) {
    println!("{b}");
    }
}
