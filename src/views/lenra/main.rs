use lenra_app::{
    components::lenra::{flex, view, FlexCrossAxisAlignment, LenraComponent, StylesDirection},
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result, props,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct LenraRouteProps {
    pub name: String,
}

props!(LenraRouteProps);

pub fn main(params: ViewParams<Value, LenraRouteProps>) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![
        view("lenra:menu").into(),
        view(params.props.unwrap().name).into(),
    ])
    .direction(StylesDirection::Vertical)
    .scroll(true)
    .spacing(4_f64)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
