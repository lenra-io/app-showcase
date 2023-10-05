use crate::{
    listeners::{COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER},
    views::lenra::{get_test_views, main::LenraRouteProps},
};
use lenra_app::{
    components::lenra::{view, ViewDefinitionsFind},
    manifest::{Exposer, Manifest, Route},
    Result,
};
use serde_json::json;

pub fn get_manifest() -> Result<Manifest> {
    let mut lenra_test_routes: Vec<Route> = get_test_views()
        .iter()
        .map(|path| {
            Route::builder()
                .path(format!("/{}", path))
                .view(
                    view("lenra:main").props(Some(
                        LenraRouteProps {
                            name: path.to_string(),
                        }
                        .try_into()
                        .unwrap(),
                    )),
                )
                .try_into()
                .unwrap()
        })
        .collect();

    let mut lenra_routes: Vec<Route> = vec![Route::builder()
        .path("/")
        .view(view("lenra:main"))
        .try_into()
        .unwrap()];
    lenra_routes.append(&mut lenra_test_routes);
    Ok(Manifest::builder()
        .json(Some(
            Exposer::builder()
                .routes(vec![
                    Route::builder()
                        .path("/counter/global")
                        .view(
                            view("json:counter").find(Some(
                                ViewDefinitionsFind::builder()
                                    .coll(COUNTER_COLLECTION)
                                    .query(json!({ "user": GLOBAL_USER }))
                                    .try_into()?,
                            )),
                        )
                        .try_into()
                        .unwrap(),
                    Route::builder()
                        .path("/counter/me")
                        .view(
                            view("json:counter").find(Some(
                                ViewDefinitionsFind::builder()
                                    .coll(COUNTER_COLLECTION)
                                    .query(json!({ "user": CURRENT_USER }))
                                    .try_into()?,
                            )),
                        )
                        .try_into()
                        .unwrap(),
                ])
                .try_into()?,
        ))
        .lenra(Some(Exposer::builder().routes(lenra_routes).try_into()?))
        .try_into()?)
}
