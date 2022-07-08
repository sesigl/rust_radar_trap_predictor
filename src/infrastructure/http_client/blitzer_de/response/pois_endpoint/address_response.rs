use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressResponse {
    pub country: String,
    pub state: String,
    pub zip_code: String,
    pub city: String,
    pub city_district: String,
    pub street: String
}