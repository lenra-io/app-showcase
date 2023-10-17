use lenra_app::{view::View, Handler};
use crate::listeners::LogProps;
use lenra_app::components::listener;
use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};

pub fn get_views() -> Vec<View> {
    vec![
        View::new("lenra/views/actionable", home),
    ]
}

fn home(_params: ViewParams) -> Result<ViewResponse> {
    let result: LenraComponent = flex(vec![actionable(
        container()
            .child(Some(Box::new(text("Test Actionable").into())))
            .decoration(Some(
                StylesBoxDecoration::builder()
                    .color(StylesColor(0xFFFFFFFF))
                    .box_shadow(Some(
                        StylesBoxShadow::builder()
                            .blur_radius(8_f64)
                            .color(StylesColor(0x1A000000))
                            .offset(Some(
                                StylesOffset::builder().dx(0_f64).dy(1_f64).try_into()?,
                            ))
                            .try_into()?,
                    ))
                    .try_into()?,
            )),
    )
    .on_pressed(Some(
        listener("log")
            .props(Some(
                LogProps {
                    message: "Clicked on actionable".to_string(),
                }
                .into(),
            ))
            .try_into()?,
    ))
    .on_double_pressed(Some(
        listener("log")
            .props(Some(
                LogProps {
                    message: "Double pressed on actionable".to_string(),
                }
                .into(),
            ))
            .try_into()?,
    ))
    .on_hovered(Some(
        listener("log")
            .props(Some(
                LogProps {
                    message: "Hovered actionable".to_string(),
                }
                .into(),
            ))
            .try_into()?,
    ))
    .on_long_pressed(Some(
        listener("log")
            .props(Some(
                LogProps {
                    message: "Long pressed on actionable".to_string(),
                }
                .into(),
            ))
            .try_into()?,
    ))
    .on_pressed_cancel(Some(
        listener("log")
            .props(Some(
                LogProps {
                    message: "Canceled click on actionable".to_string(),
                }
                .into(),
            ))
            .try_into()?,
    ))
    .into()])
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
