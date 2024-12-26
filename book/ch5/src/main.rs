// Example Program of Structs

// Allows the compiler to implemenet a basic debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height 
    }
}

//New implementation block
impl Rectangle {
    //Since it is an associated function, and not a method, do not
    // pass in the self parameter like usual
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 100,
        height: 60
    };

    let rect3 = Rectangle {
        width: 20,
        height: 40
    };

    //Associated Function
    let rect4 = Rectangle::square(25);

    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));
    println!("Rect can hold rect3: {}", rect.can_hold(&rect3));

    println!("rect {:#?}", rect);

    println!("
    The area of the rectangle is {} square pixels.", rect.area()
);
}
