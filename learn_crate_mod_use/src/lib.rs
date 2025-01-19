mod front_of_house;
use front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
    // Absolute path -> For this all the path take from the root of the crate and fo this all the items are private.
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path ->
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}
pub fn deliver_order() {
    println!("deliver_order");
}

mod back_of_house;
use crate::back_of_house::{
    cook_order, eat_at_restaurant as back_eat_at_restaurant, fix_incorrect_order, Breakfast,
};

pub fn back_house(){
    back_eat_at_restaurant();
    fix_incorrect_order();
    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    cook_order();
    
}
