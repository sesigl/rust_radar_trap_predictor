use crate::domain::country::repository::Repository;
use crate::infrastructure::persistence::country::in_memory_repository::InMemoryRepository;

mod domain;
mod infrastructure;

fn main() {

    let country_repository : InMemoryRepository = InMemoryRepository::new();
    let country_de = country_repository.find_by_country_code("DE");

    country_de.unwrap().calculate_area_cords(100).iter().for_each(|cord| println!("lat: {}, long: {}", cord.latitude, cord.longitude));
}
