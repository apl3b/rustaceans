mod back_of_house;
mod front_of_house;

fn deliver_order() {}

mod customer {
    use crate::{back_of_house, front_of_house::hosting};

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        let mut meal = back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        let order = back_of_house::Appetizer::Soup;
    }
}
