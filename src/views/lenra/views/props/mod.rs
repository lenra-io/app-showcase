use lenra_app::{view::View, Handler};

use self::{counter::counter, home::home};

mod counter;
mod home;

pub fn get_views() -> Vec<View> {
    vec![
        View::new("lenra/views/props", home),
        View::new("lenra/views/props/counter", counter),
    ]
}
