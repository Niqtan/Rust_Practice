fn main() {
    println!("Hello, world!");
}

//Example 01
fn return_a_string() -> \String {
    let s = String::from("Hello world");
    s
}

//Example 2
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name = name.clone();
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

//Example 03
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: usize = 
      dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

//Example 04

let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];
let s: String = *s_ref;

// These drops are normally implicit, but we've added them for clarity.
drop(s);
drop(v);

//Example 05: Mutating different array elements
let mut a = [0, 1, 2, 3];
let x = &mut a[1];
*x += 1;
println!("{a:?}");