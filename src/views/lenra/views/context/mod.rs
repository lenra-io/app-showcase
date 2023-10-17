use lenra_app::{view::View, Handler};

use self::{home::home, screen_size::screen_size_all};

mod home;
mod screen_size;

pub fn get_views() -> Vec<View> {
    vec![
        View::new("lenra/views/context", home),
        View::new("lenra/views/context/screen_size_all", screen_size_all),
    ]
}
