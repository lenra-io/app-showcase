use lenra_app::{
    components::{lenra::*, listener},
    props,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use serde::{Deserialize, Serialize};

use super::get_test_views;

const NAV_TO_ACTION: &str = "@lenra:navTo";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct NavToProps {
    path: String,
}

props!(NavToProps);

pub fn home(_params: ViewParams) -> Result<ViewResponse> {
    let text_views = get_test_views();

    let result: LenraComponent = flex(
        text_views
            .iter()
            .map(|text| {
                button(text)
                    .on_pressed(Some(
                        listener(NAV_TO_ACTION)
                            .props(Some(
                                NavToProps {
                                    path: format!("/{}", text),
                                }
                                .into(),
                            ))
                            .try_into()
                            .unwrap(),
                    ))
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<LenraComponent>>(),
    )
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
