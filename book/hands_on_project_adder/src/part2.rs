fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two(a: usize) -> usize {
    a + 2
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = 2 + 2;

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }

    
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    #[ignore]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

}