mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_wait_list();
  hosting::add_to_wait_list();
  crate::front_of_house::hosting::add_to_wait_list();
}
