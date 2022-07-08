use chrono::{DateTime, Utc};
use crate::domain::radar_trap::r#type::Type;
use crate::domain::radar_trap::radar_trap::RadarTrap;
use crate::domain::radar_trap::radar_trap::Address;
use crate::infrastructure::http_client::blitzer_de::response::pois_endpoint::pois_element_response::PoisElementResponse;
use crate::infrastructure::http_client::blitzer_de::response::pois_endpoint::pois_endpoint_response::PoisEndpointResponse;

pub struct BlitzerDeHttpParser {}

impl BlitzerDeHttpParser {
    pub fn fetch(&self) -> RadarTrap {

        let response: PoisEndpointResponse = reqwest::get("https://cdn3.atudo.net/api/4.0/pois.php?type=1&box=51.179342999999996,10.817169,51.799274,12.974853999999999")
            .await?
            .json::<PoisResponse>().await?;

        response.pois.map(|poi:PoisElementResponse |
            return self.toRadarTrap(poi)
        );
    }
    fn toRadarTrap(&self, poi: PoisElementResponse) -> U {
        return RadarTrap {
            id: u64::from(poi.id),
            lat: f64::from(poi.lat),
            lng: f64::from(poi.lng),
            address: Address {
                country: poi.address.country.,
                state: poi.address.state,
                zip_code: poi.address.zip_code,
                city: poi.address.city,
                city_district: poi.address.city_district,
                street: poi.address.street,
            },
            r#type: Type::DYNAMIC,
            create_date: DateTime::parse_from_str() poi.create_date,
            confirm_date: Utc::now(),

            )
    }
}