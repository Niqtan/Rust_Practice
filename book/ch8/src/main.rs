use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow")

    let mut scores = Hashmap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    //Iterate over a key value pair
    for (key, value) in &scores {
        println!("{key} {value}");
    }
}  


//Updating a hashmap

fn main() {
    let mut scores = Hashmap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    //Does not overwrite values
    scores.entry(String::from("Yellow").or_insert(30));
    scores.entry(String::from("Yellow").or_insert(40));
}

//Update a  value on hashmap based on an old value

fn main() {
    let text = "hello world world world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}