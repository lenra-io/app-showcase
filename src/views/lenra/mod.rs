use lenra_app::{view::View, Handler, RequestHandler};

use crate::views::lenra::{counter::counter, home::home, main::main, menu::menu};

mod counter;
mod home;
pub mod main;
mod menu;
mod views;

pub fn get_views() -> Vec<View> {
    let mut views: Vec<View> = vec![
        View::new("lenra:main", main),
        View::new("lenra:home", home),
        View::new("lenra:menu", menu),
        View::new("lenra:counter", counter),
    ];
    views.append(&mut views::get_views());
    views
}

pub fn get_test_views() -> Vec<String> {
    let mut views = vec![];
    views.append(&mut views::get_test_views());
    views
}

pub fn list_test_views(views: Vec<View>) -> Vec<String> {
    views.iter().map(|view| view.name()).collect()
}
