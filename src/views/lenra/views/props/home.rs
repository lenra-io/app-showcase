use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};

use crate::views::lenra::counter::CounterViewProps;

pub fn home(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![
        view("lenra/views/props/counter")
            .props(Some(
                CounterViewProps {
                    text: "1st Counter".into(),
                }
                .try_into()?,
            ))
            .try_into()?,
        view("lenra/views/props/counter")
            .props(Some(
                CounterViewProps {
                    text: "2nd Counter".into(),
                }
                .try_into()?,
            ))
            .try_into()?,
    ])
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
