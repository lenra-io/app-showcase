use lenra_app::components::lenra;
use lenra_app::{
    components::lenra::*,
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use lenra_app::{view::View, Handler};

pub fn get_views() -> Vec<View> {
    vec![View::new("lenra/views/carousel", home)]
}

fn generate_child(index: usize) -> Result<lenra::LenraComponent> {
    let res: lenra::LenraComponent = container()
        .constraints(Some(
            StylesBoxConstraints::builder()
                .min_width(100_f64)
                .min_height(100_f64)
                .max_width(100_f64)
                .max_height(100_f64)
                .try_into()?,
        ))
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
        ))
        .child(Some(Box::new(text(index.to_string()).into())))
        .into();

    Ok(res)
}

fn home(_params: ViewParams) -> Result<ViewResponse> {
    let children: [lenra::LenraComponent; 100] =
        core::array::from_fn(|i| generate_child(i).unwrap());

    let result: LenraComponent = flex(vec![carousel(children.to_vec())
        .options(Some(
            StylesCarouselOptions::builder()
                .height(100_f64)
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
