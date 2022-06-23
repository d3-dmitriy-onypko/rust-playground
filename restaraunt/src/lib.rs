mod front_of_house{
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod service {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

use core::num;

use crate::front_of_house::hosting;

mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {

    }

    mod customer {
        pub fn eat_at_the_restaraunt() {
            let mut meal = back_of_the_house::Breakfast::summer("Rya");
            meal.toast = String::from("toast");
            println!("I'd like {} toast please", meal.toast);
        }
    }
}



fn deliver_order() {
    
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort();
    let mut end_numnums.len()
}
