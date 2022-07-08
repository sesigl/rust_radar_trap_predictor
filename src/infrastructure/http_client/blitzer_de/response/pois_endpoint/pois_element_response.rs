use serde::{Deserialize, Serialize};
use crate::infrastructure::http_client::blitzer_de::response::pois_endpoint::address_response::AddressResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct PoisElementResponse {
    pub id: String,
    pub lat: String,
    pub lng: String,
    pub address: AddressResponse,
    pub content: String,
    pub backend: String,
    pub r#type: String,
    pub vmax: String,
    pub counter: String,
    pub create_date: String,
    pub confirm_date: String
}