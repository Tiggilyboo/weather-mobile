use super::api::weather::WeatherData;
use super::api::location::LocationPoint;
use super::api::units::Units;
use super::preferences::WeatherPreferences;

pub enum WeatherUpdate {
    Data(Option<WeatherData>),
    Location(Option<String>),
    SearchLocations(String),
    SetLocations(Option<Vec<LocationPoint>>),
    SavePreferences(WeatherPreferences),
    SetUnits(Units),
    Refresh,
}
