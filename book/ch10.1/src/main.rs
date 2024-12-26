
struct Point<T> {
    x: T,
    y: T,
}

//X method is available for any type
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

//y method is available for floating point types
impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}


fn st() {
    let p1 = Point {x:5, y:10}; 
    p1.x();
    let p2 = Point {x: 5.0, y: 1.0};
    p2.y();
}
*/
//More concrete examples
struct Point1<T, U> {
    x: T,
    y: U,
}

//X method is available for any type
impl<T, U> Point1<T,U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
        x: self.x, //Returns a new point  
        y: other.y,
        }
    }
}




fn main() {
    let p1 = Point1 {x:5, y:10}; 
    let p2 = Point1 {x: 5.0, y: 1.0};
    
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y= {}", p3.x, p3.y);
}