use super::api::weather::WeatherData;
use super::api::location::LocationPoint;
use super::preferences::WeatherPreferences;

pub enum WeatherUpdate {
    Data(WeatherData),
    Location(Option<String>),
    SearchLocations(String),
    SetLocations(Vec<LocationPoint>),
    SavePreferences(WeatherPreferences),
    Exit,
}
