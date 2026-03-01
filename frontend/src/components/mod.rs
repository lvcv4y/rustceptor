pub mod shared;
pub mod layout;
pub mod panels;
pub mod utils;

use yew::prelude::{Classes, classes};

pub trait ClassVariant {
    fn to_class_str(self: &Self) -> &'static str;
    fn classes(self: &Self) -> Classes {
        classes!(String::from(self.to_class_str()))
    }
}