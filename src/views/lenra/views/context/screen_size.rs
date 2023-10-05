use lenra_app::components::lenra::*;
use lenra_app::view::ViewResponseGenerator;
use lenra_app::view::{ViewParams, ViewResponse};
use lenra_app::Result;

pub fn screen_size_all(params: ViewParams) -> Result<ViewResponse> {
    let context = params.context.unwrap();
    let screen_size = context.screen_size.unwrap();
    let result: LenraComponent = text(format!(
        "Screen size {} / {}",
        screen_size.width.unwrap(),
        screen_size.height.unwrap()
    ))
    .into();
    Ok(result.gen())
}
