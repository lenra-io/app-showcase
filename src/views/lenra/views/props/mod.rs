use lenra_app::{view::View, Handler};

use crate::views::lenra::list_test_views;

use self::{counter::counter, home::home};

mod counter;
mod home;

pub fn get_views() -> Vec<View> {
    vec![
        View::new("lenra/views/props/home", home),
        View::new("lenra/views/props/counter", counter),
    ]
}

pub fn get_test_views() -> Vec<String> {
    list_test_views(get_views())
}
