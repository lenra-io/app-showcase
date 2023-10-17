use lenra_app::view::View;

// mod context;
mod find;
mod props;

pub fn get_views() -> Vec<View> {
    let mut views = vec![];
    views.append(&mut props::get_views());
    // views.append(&mut context::get_views());
    views.append(&mut find::get_views());
    views
}

pub fn get_test_views() -> Vec<String> {
    vec![
        "lenra/views/props".into(),
        // "lenra/views/context".into(),
        "lenra/views/find".into(),
    ]
}
