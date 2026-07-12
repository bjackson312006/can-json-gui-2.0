//! CAN JSON schema. Should be kept in sync with Calypso (or we prolly should just have a common crate that can be pulled in by apps).

use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An Odyssey message.
/// 
/// This is the top level object in the CAN JSON files. A CAN JSON file is basically just a vector of `OdysseyMsg` objects.
#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged, expecting = "CANMsg")]
pub enum OdysseyMsg {
    /// A normal CAN message. Like 99% of the messages in the JSON files are these.
    Can(CANMsg),

    /// A message to be indexed but not recieved.
    /// 
    /// This is basically just a subset of `CANMsg`. It is a `CANMsg`, but with no id, points, or other stuff.
    /// Because of this, `Meta` MUST GO LAST in the num. Otherwise, Serde will mess up serializing it.
    Meta(MetaMsg),
}

/// A `MetaMsg`.
/// 
/// Like a `CANMsg`, but only containing `desc` and `fields`. Exists (I think?) to register MQTT topics
/// in the system without actually corresponding to a real CAN message.
#[derive(JsonSchema, Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetaMsg {
    pub desc: String,
    pub fields: Vec<DumbNetField>,
}

/// Smart guy
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DumbNetField {
    pub name: String,
    pub unit: String,
    pub doc: String,
    pub desc: Option<String>,
}

/// Represents a CAN message.
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANMsg {
    pub id: String,
    pub desc: String,
    pub points: Vec<CANPoint>,
    pub fields: Vec<NetField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidir_mode: Option<BidirMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_freq: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<u16>>,
}

/// Mode for Calypso messages.
/// 
/// Calypso can take in MQTT commands, and turn them into CAN frames to be sent on the bus.
/// This enum lets you configure how Calypso sends those messages.
/// 
/// Note: This isn't relavent at all for most normal CAN messages that originate via firmware. This is only meaningful for messages that
/// Calypso sends.
#[derive(JsonSchema, Debug, Deserialize, Serialize, PartialEq, Copy, Clone)]
#[serde(rename_all(deserialize = "lowercase", serialize = "lowercase"))]
#[derive(Default)]
pub enum BidirMode {
    Oneshot,
    #[default]
    Broadcast,
    Configuration,
}

/// Represents a `NetField` of a CAN message.
/// 
/// A `NetField` packages one or more CANPoints into a MQTT topic, with some extra metadata.
/// A NetField can be linked to points via the `values` vector.
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NetField {
    pub name: String,
    pub unit: String,
    pub doc: String,
    pub desc: Option<String>,
    pub values: Vec<usize>,
}

/// Represents a CAN Point.
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CANPoint {
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endianness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<Formatter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ieee754_f32: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim: Option<Sim>,
}

/// Represents a CAN Point's formatter.
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
pub struct Formatter {
    /// The operation to perform (typically "divide" or "multiply").
    pub key: String,

    /// The operator's argument (i.e., how much to multiple/divide the value).
    pub arg: f32,
}

/// Represents a CAN Point's Sim configuration, for Argos purposes.
#[derive(JsonSchema, Deserialize, Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum Sim {
    SimRange {
        min: f32,
        max: f32,
        inc_min: f32,
        inc_max: f32,
        #[serde(skip_serializing_if = "Option::is_none")]
        round: Option<bool>,
    },
    SimDiscrete {
        options: Vec<[f32; 2]>,
    },
}