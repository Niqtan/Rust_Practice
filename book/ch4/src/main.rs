fn main() {
    let s2 = "hello world";
    //String literal which are string slices
    
    let word = first_word(&s2);
    //String slice

    println!("{word}");
}

fn first_word(s: &str) -> &str {
    //Returns string as an array of bytes
    let bytes = s.as_bytes();

    //i stands for the index of each item and the item itself
    for (i, &item)  in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

//Other slices

fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}