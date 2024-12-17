use std::collections::HashMap;

fn main() {
    
    let v = vec![1, 2, 3, 4, 5];

    let median = &v[2];
    println!("Median: {median}");

    let mut mode = HashMap::new();
    for vec in v {
        let count = mode.entry(vec).or_insert(0);
        *count += 1;

    }

    println!("{:?}", mode);
}
