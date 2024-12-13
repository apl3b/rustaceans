fn fx_incorrect_order() {
    cook_order();
    super::deliver_order();
}

fn cook_order() {}

pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

#[allow(dead_code)]
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
