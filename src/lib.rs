#![no_std]
pub mod public_api_types;
pub mod admi_api_types;


use serde::{Deserialize, Deserializer};

use crate::public_api_types::{list_stops_request::SearchMode, stop::Type };


impl SearchMode {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "ID" => Ok(Self::Id as i32),
            "DISTANCE" => Ok(Self::Distance as i32),
            _ => Err(serde::de::Error::unknown_variant(s, &["ID", "DISTANCE"])),
        }
    }
}

impl Type {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "STOP" => Ok(Self::Stop as i32),
            "STATION" => Ok(Self::Station as i32),
            "ENTRANCE_OR_EXIT" => Ok(Self::EntranceOrExit as i32),
            "GENERIC_NODE" => Ok(Self::GenericNode as i32),
            "BOARDING_AREA" => Ok(Self::BoardingArea as i32),
            "PLATFORM" => Ok(Self::Platform as i32),
            _ => Err(serde::de::Error::unknown_variant(s, &["STOP", "STATION", "ENTRANCE_OR_EXIT", "GENERIC_NODE", "BOARDING_AREA", "PLATFORM"])),
        }
    }
}