//Struct definitions
struct Point<T, U> {
    //Add multiple types in order for flexbility
    x: T,
    y: U,
//Generics can help by being a placeholder on a data type
}


fn main() {
    let p1 = Point {x:5, y:10};
    let p2 = Point {x: 5.0, y: 10.0};
    let p2 = Point {x: 5, y: 10.0};
}

//Enum definitions
fn enums() {
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}