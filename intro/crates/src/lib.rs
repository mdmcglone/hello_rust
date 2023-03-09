mod front_of_house;

pub use crate::front_of_house::hosting;

mod customer {
    use super::front_of_house::hosting as h;

    pub fn eat_at_restaurant() {

        h::add_to_waitlist();
    }
}