/// Haversine formula — calculates real-world distance between
/// two GPS coordinates in kilometers
pub fn haversine(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth radius in km
    let dlat = (lat2 - lat1).to_radians();
    let dlng = (lng2 - lng1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos()
        * lat2.to_radians().cos()
        * (dlng / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}