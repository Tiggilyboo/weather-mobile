use serde::Deserialize;
use isahc::prelude::*;
use super::units::Units;
use time::OffsetDateTime;

const OPEN_WEATHER_API_KEY: &str = "ad589466a7b4db65d43f8e6c850f97e5";
const OPEN_WEATHER_API_URL: &str = "https://api.openweathermap.org/data";
const OPEN_WEATHER_API_VERSION: &str = "2.5";

#[derive(Deserialize)]
pub struct WeatherMinutely {
    dt: i64, 
    pub precipitation: f64,
}

#[derive(Deserialize)]
pub struct WeatherDayTemps {
    pub day: f64,
    pub night: f64,
    pub eve: f64,
    pub morn: f64,
}

#[derive(Deserialize)]
pub struct WeatherStatus {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String
}

#[derive(Deserialize)]
pub struct WeatherAlert {
    start: i64,
    end: i64,
    pub sender_name: String,
    pub event: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct Weather<T> {
    dt: i64,
    #[serde(default)]
    pub sunrise: Option<i64>,
    #[serde(default)]
    pub sunset: Option<i64>,
    pub temp: T,
    pub feels_like: T,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: f64,
    #[serde(default)]
    pub visibility: Option<u32>,
    pub wind_speed: f64,
    pub wind_deg: u32,
    #[serde(rename = "weather")]
    pub status: Vec<WeatherStatus>,
    #[serde(default)]
    pub pop: f64,
}

pub type CurrentWeather = Weather<f64>;
pub type DailyWeather = Weather<WeatherDayTemps>;

#[derive(Deserialize)]
pub struct WeatherData {
   pub current: CurrentWeather,
   pub hourly: Vec<CurrentWeather>,
   pub minutely: Vec<WeatherMinutely>,
   pub daily: Vec<DailyWeather>,
    #[serde(default)]
   pub alerts: Vec<WeatherAlert>,
   pub units: Option<Units>,
}

pub fn display_temperature(degrees: f64, units: &Units) -> String {
    format!("{} {}", degrees, units.unit_of_measure())
}

pub fn time_from(dt: i64, format: &str) -> String {   
    let offset = OffsetDateTime::from_unix_timestamp(dt);
    let time = offset.time();
    time.format(format)
}

pub fn date_from(dt: i64) -> String {
    let offset = OffsetDateTime::from_unix_timestamp(dt);
    let date = offset.date();
    date.to_string()
}

pub fn date_time_from(dt: i64, format: &str) -> String {
    format!("{} {}", date_from(dt), time_from(dt, format))
}

pub trait TimeStamped {
    fn time(&self, format: &str) -> String;
    fn date(&self) -> String;
}

impl TimeStamped for CurrentWeather {
    fn time(&self, format: &str) -> String {
        time_from(self.dt, format)
    }
    fn date(&self) -> String {
        date_from(self.dt)
    }
}
impl TimeStamped for DailyWeather {
    fn time(&self, format: &str) -> String {
        time_from(self.dt, format)
    }
    fn date(&self) -> String {
        date_from(self.dt)
    }
}
impl TimeStamped for WeatherMinutely {
    fn time(&self, format: &str) -> String {
        time_from(self.dt, format)
    }
    fn date(&self) -> String {
        date_from(self.dt)
    }
}
impl DailyWeather {
    pub fn sunset(&self) -> Option<String> {
        if let Some(time) = self.sunset {
            Some(time_from(time, "%T"))
        } else {
            None
        }
    }
    pub fn sunrise(&self) -> Option<String> {
        if let Some(time) = self.sunrise {
            Some(time_from(time, "%T"))
        } else {
            None
        }
    }
}

fn base_url() -> String {
    return format!("{}/{}", 
        OPEN_WEATHER_API_URL, 
        OPEN_WEATHER_API_VERSION);
}

pub async fn get_weather_data(units: Units, lat: f64, lon: f64) -> Option<WeatherData> {
    let url = format!("{}/onecall?lat={}&lon={}&units={}&appid={}",
       base_url(),
       lat, lon,
       units,
       OPEN_WEATHER_API_KEY);

    let response = isahc::get_async(url).await;
    if let Some(mut body) = response.ok() {
        let text = body.text().await;
        let text = text.unwrap();
        //println!("weather got: {}", text);
        
        let mut data: WeatherData = serde_json::from_str(&text)
            .expect("Unable to deserialize weather");

        data.units = Some(units);
        Some(data)
    } else {
        None
    }
}

impl WeatherAlert {
    pub fn when(&self) -> String {
        let start = date_from(self.start);
        let start_time = time_from(self.start, "%T");
        let end = date_from(self.end);
        let end_time = time_from(self.end, "%T");

        if start != end {
            format!("{} {} to {} {}", start, start_time, end, end_time)
        } else {
            format!("{} {} to {}", start, start_time, end_time)
        }
    }
}
