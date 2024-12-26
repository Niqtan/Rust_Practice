fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
//Example 02

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}


//&i32 
// &'a i32
// &'a mut i32


fn longest<'a>(x: &str, y: &str) -> String {
    //Problem 01: x can have a longer lifetime than y
    let result = String::from("really long string");
    result
}