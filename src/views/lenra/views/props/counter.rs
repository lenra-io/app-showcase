use lenra_app::components::lenra::*;
use lenra_app::view::ViewResponseGenerator;
use lenra_app::view::{ViewParams, ViewResponse};
use lenra_app::{props, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn counter(params: ViewParams<Value, CounterViewProps>) -> Result<ViewResponse> {
    let props = params.props.unwrap();

    let result: LenraComponent = flex(vec![
        text(format!("{}: {}", props.text, 0)).try_into()?,
        button("+").into(),
    ])
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterViewProps {
    pub text: String,
}

props!(CounterViewProps);
