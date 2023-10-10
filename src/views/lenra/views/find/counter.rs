use lenra_app::api::Doc;
use lenra_app::components::{lenra::*, listener};
use lenra_app::view::ViewResponseGenerator;
use lenra_app::view::{ViewParams, ViewResponse};
use lenra_app::{props, Result};
use serde::{Deserialize, Serialize};

use crate::data::Counter;
use crate::listeners::IncrementProps;

pub fn counter(params: ViewParams<Vec<Counter>>) -> Result<ViewResponse> {
    let counters = params.data.unwrap();

    let result: LenraComponent = flex(
        counters
            .iter()
            .map(|counter| {
                flex(vec![
                    text(format!(
                        "{}: {}",
                        counter.clone().id.unwrap(),
                        counter.count
                    ))
                    .try_into()
                    .unwrap(),
                    button("+")
                        .on_pressed(Some(
                            listener("increment")
                                .props(Some(
                                    IncrementProps {
                                        id: counter.id().unwrap().clone(),
                                    }
                                    .into(),
                                ))
                                .try_into().unwrap(),
                        ))
                        .into(),
                ])
                .spacing(16_f64)
                .main_axis_alignment(FlexMainAxisAlignment::SpaceEvenly)
                .cross_axis_alignment(FlexCrossAxisAlignment::Center)
                .into()
            })
            .collect::<Vec<LenraComponent>>(),
    )
    .into();

    Ok(result.gen())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CounterViewProps {
    pub text: String,
}

props!(CounterViewProps);
