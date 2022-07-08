use chrono::{DateTime, Utc};
use crate::domain::radar_trap::address::Address;
use crate::domain::radar_trap::r#type::Type;

pub struct RadarTrap {
    pub(crate) id: u64,
    pub(crate) lat: f64,
    pub(crate) lng: f64,
    pub(crate) address: Address,
    pub(crate) r#type: Type,
    pub create_date: DateTime<Utc>,
    pub(crate) confirm_date: DateTime<Utc>
}