fn main() {
    let number_list = vec![34, 50, 25, 10, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    
    let char_list = vec!['y', 'a', 'b', 'q'];

    let largest = get_largest_char(char_list);

    println!("The largest number is {largest}");
}

//Uses a trait in order for the greater than logic gate to work 
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_char(number_list: Vec<char>) -> char {
    let mut largest = number_list[0];
    
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
