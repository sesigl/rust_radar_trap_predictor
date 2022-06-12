use crate::domain::country::coordinate::coordinate::Coordinate;
use crate::domain::country::location_box::LocationBox;

pub struct Country {
    pub name: String,
    pub code: String,
    pub location_box: LocationBox,
}

impl Country {
    pub fn calculate_area_coordinates(&self, point_distance_kilo_meters: f64) -> Vec<Coordinate> {
        // calculate latitude steps
        let distance_latitude = LocationBox {
            start: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.start.longitude },
            end: Coordinate { latitude: self.location_box.end.latitude, longitude: self.location_box.start.longitude },
        }.calculate_start_end_distance_km();

        let distance_longitude = LocationBox {
            start: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.start.longitude },
            end: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.end.longitude },
        }.calculate_start_end_distance_km();

        let latitude_iteration_count: f64 = distance_latitude / point_distance_kilo_meters;
        let latitude_iteration_count_ceil = latitude_iteration_count.floor();

        let longitude_iteration_count: f64 = distance_longitude / point_distance_kilo_meters;
        let longitude_iteration_count_ceil = longitude_iteration_count.floor();

        let latitude_degree_step: f64 = (self.location_box.start.latitude - self.location_box.end.latitude).abs() / self.min1(latitude_iteration_count_ceil);
        let longitude_degree_step: f64 = (self.location_box.start.longitude - self.location_box.end.longitude).abs() / self.min1(longitude_iteration_count_ceil);

        let mut area_coordinates: Vec<Coordinate> = vec![];
        for latitude_i in 0..(latitude_iteration_count_ceil + 1.0) as u32 {
            for longitude_i in 0..(longitude_iteration_count_ceil + 1.0) as u32 {
                let latitude = self.location_box.start.latitude + (f64::from(latitude_i) * latitude_degree_step);
                let longitude = self.location_box.start.latitude + (f64::from(longitude_i) * longitude_degree_step);

                area_coordinates.push(Coordinate {
                    longitude,
                    latitude,
                })
            }
        }

        return area_coordinates;
    }

    fn min1(&self, num: f64) -> f64 {
        return if num < 1.0 {
            1_f64
        } else {
            num
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_start_end_distance_single_coordinate_when_smaller_area_than_requested() {
        let (single_dimension_distance, country) = create_country_with_distance(0.9);

        let area_coordinates = country.calculate_area_coordinates(single_dimension_distance * 2.0);
        assert_eq!(area_coordinates.len(), 1);
        assert_eq!(area_coordinates[0].longitude, country.location_box.start.longitude);
        assert_eq!(area_coordinates[0].latitude, country.location_box.start.latitude);
    }

    #[test]
    fn calculate_start_end_distance_two_coordinates_per_latitude_step_when_bigger_area_than_requested() {
        let (single_dimension_distance, country) = create_country_with_distance(0.9);

        let area_coordinates = country.calculate_area_coordinates(single_dimension_distance - 1.0);
        assert_eq!(area_coordinates.len(), 4);
        assert_eq!(area_coordinates[0].latitude, country.location_box.start.latitude);
        assert_eq!(area_coordinates[0].longitude, country.location_box.start.longitude);
        assert_eq!(area_coordinates[1].latitude, country.location_box.start.latitude);
        assert_eq!(area_coordinates[1].longitude, country.location_box.end.longitude);
        assert_eq!(area_coordinates[2].latitude, country.location_box.end.latitude);
        assert_eq!(area_coordinates[2].longitude, country.location_box.start.longitude);
        assert_eq!(area_coordinates[3].latitude, country.location_box.end.latitude);
        assert_eq!(area_coordinates[3].longitude, country.location_box.end.longitude);
    }

    #[test]
    fn calculate_start_end_distance_multiple_coordinates_when_bigger_area_than_requested() {
        let (single_dimension_distance, country) = create_country_with_distance(0.9);

        let coordinate_step = 0.9 / 3.0;
        let area_coordinates = country.calculate_area_coordinates(single_dimension_distance / 3.0);
        assert_eq!(area_coordinates.len(), 16);

        for (i, coordinate) in area_coordinates.iter().enumerate() {
            assert_eq!(coordinate.latitude, country.location_box.start.latitude + coordinate_step * (i as f64 / 4.0).floor());
            assert_eq!(coordinate.longitude, country.location_box.start.longitude + coordinate_step * (i % 4) as f64);
        }
    }

    fn create_country_with_distance(latitude_longitude_distance: f64) -> (f64, Country) {
        let location_box = LocationBox {
            start: Coordinate { latitude: 0.0, longitude: 0.0 },
            end: Coordinate { latitude: 0.0, longitude: latitude_longitude_distance },
        };

        let single_dimension_distance = location_box.calculate_start_end_distance_km().round();

        let country = Country {
            name: "".to_string(),
            code: "".to_string(),
            location_box: LocationBox {
                start: Coordinate { latitude: 0.0, longitude: 0.0 },
                end: Coordinate { latitude: latitude_longitude_distance, longitude: latitude_longitude_distance },
            },
        };
        (single_dimension_distance, country)
    }
}