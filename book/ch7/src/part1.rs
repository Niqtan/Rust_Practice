mod front_of_house {
    mod hosting {
    fn add_to_waitlist() {}
       fn seat_at_table() {}
    }
    
    mod serving {
        fn take_order() {}
        fn serve_oder() {}
        fn take_payment() {}
    }
}



//Paths 

mod front_of_house {
  pub mod hosting {
       pub fn add_to_waitlist() {}
       }
}
pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();

        //Relative
        front_of_house::hosting::add_to_waitlist();
    }



// Starting relative paths with super

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //Allows you to specify the parent module
        super::deliver_order();
    }

    fn cook_order() {}
}


//Making structs public

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("rye");

    meal.toast = String::from("Wheat");
}


//Make enums public

mod back_of_house {
    pub enum Appetizer {
        Soup, 
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}

    

    