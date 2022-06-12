use crate::domain::country::coordinate::coordinate::Coordinate;
use crate::domain::country::country::Country;
use crate::domain::country::location_box::LocationBox;
use crate::domain::country::repository::Repository;

pub struct InMemoryRepository {
    data: Vec<Country>,
}

impl InMemoryRepository {
    pub(crate) fn new() -> InMemoryRepository {
        return InMemoryRepository {
            data: vec![
                Country {
                    name: "Germany".to_string(),
                    code: "DE".to_string(),
                    location_box: LocationBox {
                        start: Coordinate {
                            latitude: 45.283888999999995,
                            longitude: -0.027506,
                        },
                        end: Coordinate {
                            latitude: 55.398858,
                            longitude: 21.549642,
                        },
                    },
                }
            ]
        };
    }
}

impl Repository for InMemoryRepository {

    fn find_by_country_code(&self, country_code: &str) -> Option<&Country> {
        return self.data.iter().find(|&country| country.code == country_code);
    }
}