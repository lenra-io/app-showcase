use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use log::debug;

pub fn home(params: ViewParams) -> Result<ViewResponse> {
    debug!("Context: {:?}", params.context);
    let result: LenraComponent = flex(vec![view("lenra/views/context/screen_size_all")
        .context(serde_json::Map::from_iter(vec![(
            "screenSize".into(),
            true.into(),
        )]))
        .try_into()?])
    .direction(StylesDirection::Vertical)
    .spacing(16_f64)
    .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
    .cross_axis_alignment(FlexCrossAxisAlignment::Center)
    .into();
    Ok(result.gen())
}
