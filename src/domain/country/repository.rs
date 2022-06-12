use crate::domain::country::country::Country;

pub trait Repository {
    fn find_by_country_code(&self, country_code: &str) -> Option<&Country>;
}