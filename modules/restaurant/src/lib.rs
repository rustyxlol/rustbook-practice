mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Wheat");
    meal.toast = String::from("Rye");

    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
    // Wont work because it isn't public

}