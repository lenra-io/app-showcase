use crate::listeners::LogProps;
use lenra_app::components::listener;
use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use lenra_app::{view::View, Handler};

pub fn get_views() -> Vec<View> {
    vec![View::new("lenra/views/button", home)]
}

fn home(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![
        button("Basic button")
            .on_pressed(Some(
                listener("log")
                    .props(Some(
                        LogProps {
                            message: "Clicked on button".to_string(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .left_icon(Some(icon(StylesIconName::AlarmOn).try_into()?))
            .right_icon(Some(icon(StylesIconName::AlarmOff).try_into()?))
            .into(),
        button("Disabled Button").disabled(true).into(),
        button("Secondary button")
            .on_pressed(Some(
                listener("log")
                    .props(Some(
                        LogProps {
                            message: "Clicked on secondary button".to_string(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .main_style(StylesStyle::Secondary)
            .into(),
        button("Tertiary")
            .on_pressed(Some(
                listener("log")
                    .props(Some(
                        LogProps {
                            message: "Clicked on tertiary button".to_string(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .main_style(StylesStyle::Tertiary)
            .into(),
        button("Small")
            .on_pressed(Some(
                listener("log")
                    .props(Some(
                        LogProps {
                            message: "Clicked on small button".to_string(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .size(StylesSize::Small)
            .into(),
        button("Large")
            .on_pressed(Some(
                listener("log")
                    .props(Some(
                        LogProps {
                            message: "Clicked on large button".to_string(),
                        }
                        .into(),
                    ))
                    .try_into()?,
            ))
            .size(StylesSize::Large)
            .into(),
    ])
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
