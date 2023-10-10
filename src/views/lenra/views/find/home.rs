use lenra_app::{
    components::{lenra::{ViewDefinitionsFind, *}, listener},
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use serde_json::json;

use crate::listeners::{COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER, AddCounterProps};

pub fn home(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![
        find_test("No query", None)?.into(),
        find_test(
            "User counters",
            Some(
                ViewDefinitionsFind::builder()
                    .coll(COUNTER_COLLECTION)
                    .query(json!({ "user": CURRENT_USER }))
                    .try_into()?,
            ),
        )?
        .into(),
        find_test(
            "Global counters",
            Some(
                ViewDefinitionsFind::builder()
                    .coll(COUNTER_COLLECTION)
                    .query(json!({ "user": GLOBAL_USER }))
                    .try_into()?,
            ),
        )?
        .into(),
        button("Add counter")
            .on_pressed(Some(
                listener("addCounter")
                    .props(Some(AddCounterProps {
                        user: CURRENT_USER.to_string(),
                        ..Default::default()
                    }.into()))
                    .try_into()?,
            ))
            .into(),
    ])
    .direction(StylesDirection::Vertical)
    .spacing(32_f64)
    // .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    // .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}

fn find_test(name: &str, query: Option<ViewDefinitionsFind>) -> Result<LenraComponent> {
    let mut view = view("lenra/views/find/counter");
    if let Some(query) = query {
        view = view.find(Some(query.try_into()?));
    }
    Ok(flex(vec![text(name).into(), view.try_into()?])
        .direction(StylesDirection::Vertical)
        .spacing(8_f64)
        // .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
        // .cross_axis_alignment(FlexCrossAxisAlignment::Center)
        .into())
}
