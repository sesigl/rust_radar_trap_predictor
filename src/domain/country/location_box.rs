use crate::domain::country::coordinate::coordinate::Coordinate;

pub struct LocationBox {
    pub start: Coordinate,
    pub end: Coordinate,
}

impl LocationBox {
    const RADIUS_EARTH_KM: f64 = 6371_f64;

    pub fn calculate_start_end_distance_km(&self) -> f64 {
        let start_latitude = self.start.latitude.to_radians();
        let end_latitude = self.end.latitude.to_radians();

        let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
        let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();

        let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
            + start_latitude.cos() * end_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
        let central_angle = 2.0 * central_angle_inner.sqrt().asin();

        return LocationBox::RADIUS_EARTH_KM * central_angle;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0.0, 0.0, 1.0, 0.0, 111.0 ; "latitude positive increment")]
    #[test_case(0.0, 0.0, 0.0, 1.0, 111.0 ; "longitude positive increment")]
    #[test_case(1.0, 0.0, 0.0, 0.0, 111.0 ; "latitude positive decrement")]
    #[test_case(0.0, 1.0, 0.0, 0.0, 111.0 ; "longitude positive decrement")]

    #[test_case(0.0, 0.0, -1.0, 0.0, 111.0 ; "negative latitude positive increment")]
    #[test_case(0.0, 0.0, 0.0, -1.0, 111.0 ; "negative longitude positive increment")]
    #[test_case(-1.0, 0.0, 0.0, 0.0, 111.0 ; "negative latitude positive decrement")]
    #[test_case(0.0, -1.0, 0.0, 0.0, 111.0 ; "negative longitude positive decrement")]
    fn calculate_start_end_distance_km_basics(latitude_start: f64, longitude_start: f64, latitude_end: f64, longitude_end: f64, distance: f64) {
        let location_box = LocationBox {
            start: Coordinate { latitude: latitude_start, longitude: longitude_start },
            end: Coordinate { latitude: latitude_end, longitude: longitude_end },
        };

        let distance_km = location_box.calculate_start_end_distance_km();
        assert_eq!(distance_km.round(), distance);
    }

    #[test]
    fn calculate_start_end_distance_km_berlin_munich() {
        let location_box = LocationBox {
            start: Coordinate { latitude: 52.520008, longitude: 13.404954 },
            end: Coordinate { latitude: 48.137154, longitude: 11.576124 },
        };

        let distance_km = location_box.calculate_start_end_distance_km();
        assert_eq!(distance_km.round(), 504.0);

        let distance_km = location_box.calculate_start_end_distance_km();
        assert_eq!(distance_km.round(), 504.0);
    }

    #[test]
    fn calculate_start_end_distance_km_berlin_sao_paulo() {
        let location_box = LocationBox {
            start: Coordinate { latitude: 52.520008, longitude: 13.404954 },
            end: Coordinate { latitude: -23.944841, longitude: -46.330376 },
        };

        let distance_km = location_box.calculate_start_end_distance_km();
        assert_eq!(distance_km.round(), 10274.0);

        let distance_km = location_box.calculate_start_end_distance_km();
        assert_eq!(distance_km.round(), 10274.0);
    }
}