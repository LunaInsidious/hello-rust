mod front_of_house {
  pub mod hosting {
    pub fn add_to_wait_list() {}

    fn _seat_at_table() {}
  }

  mod servicing {
    fn _take_order() {}

    fn _serve_order() {}

    fn _take_payment() {}
  }
}

fn _serve_order() {}

mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
  }

  pub enum Appetizer {
    Soup,
    Salad,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        _seasonal_fruit: String::from("peaches"),
      }
    }
  }

  fn _fix_incorrect_order() {
    _cook_order();
    super::_serve_order();
  }

  fn _cook_order() {}
}

//  relative path
use self::front_of_house::hosting;
// absolute path
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // やっぱり別のパンにする
  meal.toast = String::from("Wheat");
  println!("The meal is ready with {} toast.", meal.toast);

  let _order1 = back_of_house::Appetizer::Soup;
  let _order2 = back_of_house::Appetizer::Salad;

  hosting::add_to_wait_list();
}
