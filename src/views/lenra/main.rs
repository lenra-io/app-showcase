use lenra_app::{
    components::lenra::{
        flex, view, FlexCrossAxisAlignment, LenraComponent, StylesDirection,
    },
    props,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct LenraRouteProps {
    pub name: String,
}

props!(LenraRouteProps);

pub fn main(params: ViewParams<Value, LenraRouteProps>) -> Result<ViewResponse> {
    let view_name = if let Some(props) = params.props {
        props.name
    } else {
        "lenra:home".into()
    };
    let result: LenraComponent = flex(vec![view("lenra:menu").into(), view(view_name).into()])
        .direction(StylesDirection::Vertical)
        .scroll(true)
        .spacing(4_f64)
        .cross_axis_alignment(FlexCrossAxisAlignment::Center)
        .into();
    Ok(result.gen())
}
