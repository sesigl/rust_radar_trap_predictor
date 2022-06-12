use crate::domain::country::coordinate::coordinate::Coordinate;
use crate::domain::country::location_box::LocationBox;

pub struct Country {
    pub name: String,
    pub code: String,
    pub location_box: LocationBox,
}

impl Country {

    pub fn calculate_area_cords(&self, point_distance_kilo_meters: u16) -> Vec<Coordinate> {
        // calculate latitude steps
        let distance_latitude = LocationBox {
            start: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.start.longitude },
            end: Coordinate { latitude: self.location_box.end.latitude, longitude: self.location_box.start.longitude },
        }.calculate_start_end_distance_km();

        let distance_longitude = LocationBox {
            start: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.start.longitude },
            end: Coordinate { latitude: self.location_box.start.latitude, longitude: self.location_box.end.longitude },
        }.calculate_start_end_distance_km();

        let latitude_iteration_count: f64 = distance_latitude / f64::from(point_distance_kilo_meters);
        let latitude_iteration_count_ceil = latitude_iteration_count.ceil() as u32;

        let longitude_iteration_count: f64 = distance_longitude / f64::from(point_distance_kilo_meters);
        let longitude_iteration_count_ceil = longitude_iteration_count.ceil() as u32;

        let latitude_degree_step: f64 = (self.location_box.start.latitude - self.location_box.end.latitude).abs() / latitude_iteration_count;
        let longitude_degree_step: f64 = (self.location_box.start.longitude - self.location_box.end.longitude).abs() / latitude_iteration_count;

        let mut area_coordinates: Vec<Coordinate> = vec![];
        for latitude_i in 0..latitude_iteration_count_ceil - 1 {
            for longitude_i in 0..longitude_iteration_count_ceil - 1 {
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
}