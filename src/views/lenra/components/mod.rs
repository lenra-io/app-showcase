use lenra_app::view::View;

// mod context;
mod actionable;
mod button;
mod carousel;

pub fn get_views() -> Vec<View> {
    let mut views = vec![];
    views.append(&mut actionable::get_views());
    views.append(&mut button::get_views());
    views.append(&mut carousel::get_views());
    // views.append(&mut context::get_views());
    views
}

pub fn get_test_views() -> Vec<String> {
    vec!["lenra/views/actionable".into(), "lenra/views/button".into(), "lenra/views/carousel".into()]
}
