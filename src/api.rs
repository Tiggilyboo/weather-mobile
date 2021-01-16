pub mod units;

use serde::Deserialize;
use isahc::prelude::*;
use units::Units;

const OPEN_WEATHER_API_KEY: &str = "ad589466a7b4db65d43f8e6c850f97e5";
const OPEN_WEATHER_API_URL: &str = "https://api.openweathermap.org/data";
const OPEN_WEATHER_API_VERSION: &str = "2.5";

#[derive(Deserialize)]
pub struct WeatherStatus {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Deserialize)]
pub struct Weather {
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: f32,
    pub visibility: u32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    #[serde(rename = "weather")]
    pub status: Vec<WeatherStatus>,
}

#[derive(Deserialize)]
pub struct WeatherData {
   pub current: Weather,
   pub hourly: Vec<Weather>,
   units: Option<Units>,
}

impl WeatherData {
    pub fn display_temperature(&self, degrees: f32) -> String {
        if let Some(uom) = &self.units {
            format!("{} {}", degrees, uom.unit_of_measure())
        } else {
            println!("No units set on WeatherData");
            String::new()
        }
    }
}

fn base_url() -> String {
    return format!("{}/{}", 
        OPEN_WEATHER_API_URL, 
        OPEN_WEATHER_API_VERSION);
}

pub fn get_weather_data(units: Units, lat: f32, lon: f32) -> WeatherData {
    let url = format!("{}/onecall?lat={}&lon={}&units={}&appid={}",
       base_url(),
       lat, lon,
       units,
       OPEN_WEATHER_API_KEY);

    let response = isahc::get(url);
    if let Some(mut body) = response.ok() {
        let text = body.text().unwrap();
        
        let mut data: WeatherData = serde_json::from_str(&text)
            .expect("Unable to deserialize weather");

        data.units = Some(units);
        data
    } else {
        panic!()
    }
}

