
//Iterator method definition
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    //Item type is used to return the type of next method

    // methods with default implementations elided
}


fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    //Doesnt do anything yet

    for val in v1_iter {
        println!("Got: {val}");
    }
}




//Iterator under the hood
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

//Sum method consuems the iterator
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

//Collects the iterator (consumes)
#[test]
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);

//Closures to capture iterator's environment
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
