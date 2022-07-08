use std::collections::HashMap;

use crate::domain::country::repository::Repository;
use crate::infrastructure::persistence::country::in_memory_repository::InMemoryRepository;

mod domain;
mod infrastructure;








#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let country_repository : InMemoryRepository = InMemoryRepository::new();
    let country_de = country_repository.find_by_country_code("DE");

    let country_coordinates = country_de.unwrap().calculate_area_coordinates(100.0);

    let resp = reqwest::get("https://cdn3.atudo.net/api/4.0/pois.php?type=1&box=51.179342999999996,10.817169,51.799274,12.974853999999999")
        .await?
        .text().await?;

    println!("{:#?}", resp);



    println!("{:#?}", resp);
    Ok(())
}