use super::api::weather::WeatherData;
use super::api::location::LocationPoint;

pub enum WeatherUpdate {
    Data(WeatherData),
    Location(Option<String>),
    SearchLocations(String),
    SetLocations(Vec<LocationPoint>),
    Exit,
}
