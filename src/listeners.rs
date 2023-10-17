use lenra_app::{
    api::{Api, CollectionGetter},
    components::lenra::{DataProjection, DataQuery},
    listener::{Listener, ListenerParams, SystemEvents},
    props, Handler, Result,
};
use log::debug;
// use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::data::Counter;

pub const COUNTER_COLLECTION: &str = "counter";
pub const GLOBAL_USER: &str = "global";
pub const CURRENT_USER: &str = "@me";

pub fn get_listeners() -> Vec<Listener> {
    vec![
        Listener::new(
            SystemEvents::OnEnvStart.to_str(),
            |params: ListenerParams| create_counter_if_not_exists(&params.api, GLOBAL_USER),
        ),
        Listener::new(
            SystemEvents::OnUserFirstJoin.to_str(),
            |params: ListenerParams| create_counter_if_not_exists(&params.api, CURRENT_USER),
        ),
        Listener::new(
            SystemEvents::OnSessionStart.to_str(),
            |_params: ListenerParams| Ok(()),
        ),
        Listener::new("increment", increment),
        Listener::new("addCounter", add_counter),
        Listener::new("log", log_to_console),
    ]
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct IncrementProps {
    pub id: String,
}
props!(IncrementProps);

fn increment(params: ListenerParams<IncrementProps, Value>) -> Result<()> {
    params
        .api
        .data
        .coll(COUNTER_COLLECTION)
        .update_many::<Counter, DataQuery, Value>(
            serde_json::from_value(json!({ "_id": params.props.unwrap().id }))?,
            json!({
                "$inc": {
                    "count": 1
                }
            }),
        )?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct AddCounterProps {
    pub user: String,
    pub count: Option<u32>,
}
props!(AddCounterProps);

fn add_counter(params: ListenerParams<AddCounterProps, Value>) -> Result<()> {
    let props = params.props.unwrap();
    create_counter(&params.api, props.user.clone(), props.count.clone())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct LogProps {
    pub message: String,
}
props!(LogProps);

fn log_to_console(params: ListenerParams<LogProps, Value>) -> Result<()> {
    debug!("{}", params.props.unwrap().message);
    Ok(())
}

fn create_counter_if_not_exists(api: &Api, user: &str) -> Result<()> {
    let coll = api.data.coll(COUNTER_COLLECTION);
    let query: DataQuery = serde_json::from_value(json!({ "user": user }))?;
    let counters: Vec<Counter> = coll.find(query, None::<DataProjection>)?;
    if counters.is_empty() {
        create_counter(api, user.to_string(), None)?;
    }
    Ok(())
}

fn create_counter(api: &Api, user: String, count: Option<u32>) -> Result<()> {
    api.data.coll(COUNTER_COLLECTION).create_doc(Counter {
        count: count.unwrap_or(0),
        user: user.into(),
        ..Default::default()
    })?;
    Ok(())
}
