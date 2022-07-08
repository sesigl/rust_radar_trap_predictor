use serde::{Deserialize, Serialize};
use crate::infrastructure::http_client::blitzer_de::response::pois_endpoint::pois_element_response::PoisElementResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct PoisEndpointResponse {
    pub pois: Vec<PoisElementResponse>,
}