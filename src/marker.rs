use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Marker {
    Conversion { source: String, target: String },
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Marker {
//     schema_version: u32,
//
// }
