// #![no_std]
pub mod admi_api_types;
pub mod public_api_types;

extern crate alloc;

use alloc::fmt::Display;

use serde::{Deserialize, Deserializer};
use alloc::str::FromStr;

use crate::public_api_types::{
    alert::{Cause, Effect},
    route::{ContinuousPolicy, Type as RouteType},
    stop::Type,
    system::Status,
    transfer::Type as TransferType,
    vehicle::{CongestionLevel, CurrentStatus, OccupancyStatus},
};

pub fn deserialize_number_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt<T> {
        String(String),
        Number(T),
    }

    match StringOrInt::<T>::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrInt::Number(i) => Ok(i),
    }
}

pub fn callback_i64_to_option<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let result = deserialize_number_from_string::<i64, D>(deserializer)?;
    Ok(Some(result))   
}

impl TransferType {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "RECOMMENDED" => Ok(Self::Recommended as i32),
            "TIMED" => Ok(Self::Timed as i32),
            "REQUIRES_TIME" => Ok(Self::RequiresTime as i32),
            "NOT_POSSIBLE" => Ok(Self::NotPossible as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &["RECOMMENDED", "TIMED", "REQUIRES_TIME", "NOT_POSSIBLE"],
            )),
        }
    }
}

impl Effect {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "UNKNOWN_EFFECT" => Ok(Self::UnknownEffect as i32),
            "NO_SERVICE" => Ok(Self::NoService as i32),
            "REDUCED_SERVICE" => Ok(Self::ReducedService as i32),
            "SIGNIFICANT_DELAYS" => Ok(Self::SignificantDelays as i32),
            "DETOUR" => Ok(Self::Detour as i32),
            "ADDITIONAL_SERVICE" => Ok(Self::AdditionalService as i32),
            "MODIFIED_SERVICE" => Ok(Self::ModifiedService as i32),
            "OTHER_EFFECT" => Ok(Self::OtherEffect as i32),
            "STOP_MOVED" => Ok(Self::StopMoved as i32),
            "NO_EFFECT" => Ok(Self::NoEffect as i32),
            "ACCESSIBILITY_ISSUE" => Ok(Self::AccessibilityIssue as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "UNKNOWN_EFFECT",
                    "NO_SERVICE",
                    "REDUCED_SERVICE",
                    "SIGNIFICANT_DELAYS",
                    "DETOUR",
                    "ADDITIONAL_SERVICE",
                    "MODIFIED_SERVICE",
                    "OTHER_EFFECT",
                    "STOP_MOVED",
                    "NO_EFFECT",
                    "ACCESSIBILITY_ISSUE",
                ],
            )),
        }
    }
}

impl Cause {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "UNKNOWN_CAUSE" => Ok(Self::UnknownCause as i32),
            "OTHER_CAUSE" => Ok(Self::OtherCause as i32),
            "TECHNICAL_PROBLEM" => Ok(Self::TechnicalProblem as i32),
            "STRIKE" => Ok(Self::Strike as i32),
            "DEMONSTRATION" => Ok(Self::Demonstration as i32),
            "ACCIDENT" => Ok(Self::Accident as i32),
            "HOLIDAY" => Ok(Self::Holiday as i32),
            "WEATHER" => Ok(Self::Weather as i32),
            "MAINTENANCE" => Ok(Self::Maintenance as i32),
            "CONSTRUCTION" => Ok(Self::Construction as i32),
            "POLICE_ACTIVITY" => Ok(Self::PoliceActivity as i32),
            "MEDICAL_EMERGENCY" => Ok(Self::MedicalEmergency as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "UNKNOWN_CAUSE",
                    "OTHER_CAUSE",
                    "TECHNICAL_PROBLEM",
                    "STRIKE",
                    "DEMONSTRATION",
                    "ACCIDENT",
                    "HOLIDAY",
                    "WEATHER",
                    "MAINTENANCE",
                    "CONSTRUCTION",
                    "POLICE_ACTIVITY",
                    "MEDICAL_EMERGENCY",
                ],
            )),
        }
    }
}

impl RouteType {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "LIGHT_RAIL" => Ok(Self::LightRail as i32),
            "SUBWAY" => Ok(Self::Subway as i32),
            "RAIL" => Ok(Self::Rail as i32),
            "BUS" => Ok(Self::Bus as i32),
            "FERRY" => Ok(Self::Ferry as i32),
            "CABLE_TRAM" => Ok(Self::CableTram as i32),
            "AERIAL_LIFT" => Ok(Self::AerialLift as i32),
            "FUNICULAR" => Ok(Self::Funicular as i32),
            "TROLLEY_BUS" => Ok(Self::TrolleyBus as i32),
            "MONORAIL" => Ok(Self::Monorail as i32),
            "UNKNOWN" => Ok(Self::Unknown as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "LIGHT_RAIL",
                    "SUBWAY",
                    "RAIL",
                    "BUS",
                    "FERRY",
                    "CABLE_TRAM",
                    "AERIAL_LIFT",
                    "FUNICULAR",
                    "TROLLEY_BUS",
                    "MONORAIL",
                    "UNKNOWN",
                ],
            )),
        }
    }
}

impl ContinuousPolicy {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "ALLOWED" => Ok(Self::Allowed as i32),
            "NOT_ALLOWED" => Ok(Self::NotAllowed as i32),
            "PHONE_AGENCY" => Ok(Self::PhoneAgency as i32),
            "COORDINATE_WITH_DRIVER" => Ok(Self::CoordinateWithDriver as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "ALLOWED",
                    "NOT_ALLOWED",
                    "PHONE_AGENCY",
                    "COORDINATE_WITH_DRIVER",
                ],
            )),
        }
    }
}


impl OccupancyStatus {

    pub fn option_from_str<'de, D>(deserializer: D) -> core::option::Option<i32>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer).unwrap();

        match s {
            "EMPTY" => Some(Self::Empty as i32),
            "MANY_SEATS_AVAILABLE" => Some(Self::ManySeatsAvailable as i32),
            "FEW_SEATS_AVAILABLE" => Some(Self::FewSeatsAvailable as i32),
            "STANDING_ROOM_ONLY" => Some(Self::StandingRoomOnly as i32),
            "CRUSHED_STANDING_ROOM_ONLY" => Some(Self::CrushedStandingRoomOnly as i32),
            "FULL" => Some(Self::Full as i32),
            "NOT_ACCEPTING_PASSENGERS" => Some(Self::NotAcceptingPassengers as i32),
            _ => None,
        }
    }


    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "EMPTY" => Ok(Self::Empty as i32),
            "MANY_SEATS_AVAILABLE" => Ok(Self::ManySeatsAvailable as i32),
            "FEW_SEATS_AVAILABLE" => Ok(Self::FewSeatsAvailable as i32),
            "STANDING_ROOM_ONLY" => Ok(Self::StandingRoomOnly as i32),
            "CRUSHED_STANDING_ROOM_ONLY" => Ok(Self::CrushedStandingRoomOnly as i32),
            "FULL" => Ok(Self::Full as i32),
            "NOT_ACCEPTING_PASSENGERS" => Ok(Self::NotAcceptingPassengers as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "EMPTY",
                    "MANY_SEATS_AVAILABLE",
                    "FEW_SEATS_AVAILABLE",
                    "STANDING_ROOM_ONLY",
                    "CRUSHED_STANDING_ROOM_ONLY",
                    "FULL",
                    "NOT_ACCEPTING_PASSENGERS",
                ],
            )),
        }
    }
}

impl CongestionLevel {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "UNKNOWN_CONGESTION_LEVEL" => Ok(Self::UnknownCongestionLevel as i32),
            "RUNNING_SMOOTHLY" => Ok(Self::RunningSmoothly as i32),
            "STOP_AND_GO" => Ok(Self::StopAndGo as i32),
            "CONGESTION" => Ok(Self::Congestion as i32),
            "SEVERE_CONGESTION" => Ok(Self::SevereCongestion as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "UNKNOWN_CONGESTION_LEVEL",
                    "RUNNING_SMOOTHLY",
                    "STOP_AND_GO",
                    "CONGESTION",
                    "SEVERE_CONGESTION",
                ],
            )),
        }
    }
}

impl CurrentStatus {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "INCOMING_AT" => Ok(Self::IncomingAt as i32),
            "STOPPED_AT" => Ok(Self::StoppedAt as i32),
            "IN_TRANSIT_TO" => Ok(Self::InTransitTo as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &["INCOMING_AT", "STOPPED_AT", "IN_TRANSIT_TO"],
            )),
        }
    }
}

impl Status {
    pub fn from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        match s {
            "UNKNOWN" => Ok(Self::Unknown as i32),
            "INSTALLING" => Ok(Self::Installing as i32),
            "ACTIVE" => Ok(Self::Active as i32),
            "INSTALL_FAILED" => Ok(Self::InstallFailed as i32),
            "UPDATING" => Ok(Self::Updating as i32),
            "UPDATE_FAILED" => Ok(Self::UpdateFailed as i32),
            "DELETING" => Ok(Self::Deleting as i32),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &["UNKNOWN", "INSTALLING", "ACTIVE", "INSTALL_FAILED"],
            )),
        }
    }
}

impl crate::public_api_types::list_vehicles_request::SearchMode {
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

impl crate::public_api_types::list_stops_request::SearchMode {
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
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "STOP",
                    "STATION",
                    "ENTRANCE_OR_EXIT",
                    "GENERIC_NODE",
                    "BOARDING_AREA",
                    "PLATFORM",
                ],
            )),
        }
    }
}
