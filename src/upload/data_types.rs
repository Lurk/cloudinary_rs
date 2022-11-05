use core::fmt;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use super::{
    access_mode::AccessModes, allowed_headers::AllowedHeaders,
    background_removal::BackgroundRemoval, categorizations::Categorizations,
    delivery_type::DeliveryType, raw_convert::RawConvert, resource_type::ResourceTypes,
    responsive_breakpoints::ResponsiveBreakpoints,
};

pub type Coordinates = [u32; 4];
#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Boolean(bool),
    DeliveryType(DeliveryType),
    AccessModes(AccessModes),
    ResourceTypes(ResourceTypes),
    HasSet(HashSet<String>),
    HashMap(HashMap<String, String>),
    Float(f32),
    ResponsiveBreakpoints(Vec<ResponsiveBreakpoints>),
    Categorization(Vec<Categorizations>),
    Coordinates(Coordinates),
    FaceCoordinates(Vec<Coordinates>),
    BackgroundRemoval(BackgroundRemoval),
    RawConvert(RawConvert),
    VecOfString(Vec<String>),
    AllowedHeaders(HashMap<AllowedHeaders, String>),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::String(value) => write!(f, "{}", value),
            DataType::Boolean(value) => write!(f, "{}", value),
            DataType::DeliveryType(value) => write!(f, "{}", value),
            DataType::AccessModes(value) => write!(f, "{}", value),
            DataType::ResourceTypes(value) => write!(f, "{}", value),
            DataType::HasSet(value) => write!(f, "{}", value.iter().join(",")),
            DataType::HashMap(value) => write!(
                f,
                "{}",
                value.iter().map(|(k, v)| format!("{k}={v}")).join("|")
            ),
            DataType::Float(value) => write!(f, "{}", value),
            DataType::ResponsiveBreakpoints(value) => write!(
                f,
                "['{}']",
                value
                    .iter()
                    .map(|breakpoint| serde_json::to_string(breakpoint).unwrap())
                    .join("', '")
            ),
            DataType::Categorization(value) => write!(f, "{}", value.iter().join(",")),
            DataType::Coordinates(value) => write!(f, "{}", value.iter().join(",")),
            DataType::FaceCoordinates(value) => {
                write!(f, "{}", value.iter().map(|c| c.iter().join(",")).join("|"))
            }
            DataType::BackgroundRemoval(value) => write!(f, "{}", value),
            DataType::RawConvert(value) => write!(f, "{}", value),
            DataType::VecOfString(value) => write!(f, "{}", value.iter().join(",")),
            DataType::AllowedHeaders(value) => write!(
                f,
                "{}",
                value.iter().map(|(k, v)| format!("{k}: {v}")).join("\n")
            ),
        }
    }
}
