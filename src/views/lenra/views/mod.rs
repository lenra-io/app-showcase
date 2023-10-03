use lenra_app::view::View;

mod props;

pub fn get_views() -> Vec<View> {
    let mut views = vec![];
    views.append(&mut props::get_views());
    views
}

pub fn get_test_views() -> Vec<String> {
    let mut views = vec![];
    views.append(&mut props::get_test_views());
    views
}
