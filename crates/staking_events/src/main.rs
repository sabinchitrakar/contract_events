
mod db;
use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "event_docs")]
pub struct EventDoc {
    name:String
}
fn main() {
    println!("Hello, world!");
    db::get_connection::<EventDoc>("./data/events".to_owned()).unwrap();
}
