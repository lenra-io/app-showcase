use lenra_app::{
    components::{
        lenra::{ViewDefinitionsFind, *},
        listener,
    },
    view::{ViewParams, ViewResponse, ViewResponseGenerator},
    Result,
};
use serde_json::json;

use crate::listeners::{AddCounterProps, COUNTER_COLLECTION, CURRENT_USER, GLOBAL_USER};

pub fn home(_params: ViewParams) -> Result<ViewResponse> {
    let mut children: Vec<LenraComponent> = vec![flex(
        vec![CURRENT_USER, GLOBAL_USER]
            .iter()
            .map(|user| {
                button(format!("Add {} counter", user))
                    .on_pressed(Some(
                        listener("addCounter")
                            .props(Some(
                                AddCounterProps {
                                    user: user.to_string(),
                                    ..Default::default()
                                }
                                .into(),
                            ))
                            .try_into()
                            .unwrap(),
                    ))
                    .into()
            })
            .collect::<Vec<LenraComponent>>(),
    )
    .into(),
    find_test("No query", None)?.into()];

    // $eq tests
    children.append(&mut vec![test_group("$eq", vec![("User counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({ "user": CURRENT_USER }))
            .try_into()?,
    )),
    ("Global counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({ "user": GLOBAL_USER }))
            .try_into()?,
    )),
    ("Unkown user counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({ "user": "not exists" }))
            .try_into()?,
    ))])?]);

    // $ne tests
    children.append(&mut vec![test_group("$ne", vec![("Not User counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({"user": { "$ne": CURRENT_USER }}))
            .try_into()?,
    )),
    ("Not Global counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({ "user": { "$ne": GLOBAL_USER }}))
            .try_into()?,
    )),
    ("Not Unkown user counters",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({ "user": { "$ne": "not exists" }}))
            .try_into()?,
    ))])?]);

    // $gt tests
    children.append(&mut vec![test_group("$gt", vec![("Greater than 10",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({"count": { "$gt": 10 }}))
            .try_into()?,
    ))])?]);

    // $gte tests
    children.append(&mut vec![test_group("$gte", vec![("Greater or equal 10",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({"count": { "$gte": 10 }}))
            .try_into()?,
    ))])?]);

    // $lt tests
    children.append(&mut vec![test_group("$lt", vec![("Lower than 10",
    Some(
        ViewDefinitionsFind::builder()
            .coll(COUNTER_COLLECTION)
            .query(json!({"count": { "$lt": 10 }}))
            .try_into()?,
    ))])?]);

    let result: LenraComponent = flex(children)
    .direction(StylesDirection::Vertical)
    .spacing(32_f64)
    .into();
    Ok(result.gen())
}

fn test_group(name: &str, tests: Vec<(&str, Option<ViewDefinitionsFind>)>) -> Result<LenraComponent> {
    let mut children: Vec<LenraComponent> = vec![text(name).into()];
    children.append(tests.iter().map(|(name, query)| find_test(name, query.clone()).unwrap()).collect::<Vec<LenraComponent>>().as_mut());
    Ok(flex(children)
        .direction(StylesDirection::Vertical)
        .spacing(16_f64)
        .into())
}

fn find_test(name: &str, query: Option<ViewDefinitionsFind>) -> Result<LenraComponent> {
    let mut view = view("lenra/views/find/counter");
    if let Some(query) = query {
        view = view.find(Some(query.try_into()?));
    }
    Ok(flex(vec![text(name).into(), view.try_into()?])
        .direction(StylesDirection::Vertical)
        .spacing(8_f64)
        .into())
}
