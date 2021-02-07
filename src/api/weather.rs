use serde::Deserialize;
use isahc::prelude::*;
use super::units::Units;
use time::{
    OffsetDateTime,
    UtcOffset,
};

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
pub struct Weather<T, P> {
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
    #[serde(default)]
    pub wind_gust: Option<f64>,
    #[serde(rename = "weather")]
    pub status: Vec<WeatherStatus>,
    #[serde(default)]
    pub pop: f64,
    #[serde(default)]
    pub rain: Option<P>,
    #[serde(default)]
    pub snow: Option<P>,
}

#[derive(Deserialize, Default)]
pub struct PrecipitationHourly {
    #[serde(default)]
    #[serde(rename = "1h")]
    pub volume: Option<f64>,
}

pub type CurrentWeather = Weather<f64, PrecipitationHourly>;
pub type HourlyWeather = Weather<f64, PrecipitationHourly>;
pub type DailyWeather = Weather<WeatherDayTemps, f64>;

#[derive(Deserialize)]
pub struct WeatherData {
   pub current: CurrentWeather,
   pub hourly: Vec<HourlyWeather>,
   pub minutely: Vec<WeatherMinutely>,
   pub daily: Vec<DailyWeather>,
    #[serde(default)]
   pub alerts: Vec<WeatherAlert>,
   pub units: Option<Units>,
}

pub fn time_from(dt: i64, format: &str) -> String {   
    let datetime = datetime_from(dt);
    datetime.time().format(format)
}

pub fn current_utc_offset() -> UtcOffset {
    if let Ok(offset) = UtcOffset::try_current_local_offset() {
        offset
    } else {
        UtcOffset::UTC
    }
}
pub fn datetime_from(dt: i64) -> OffsetDateTime {
    let datetime = OffsetDateTime::from_unix_timestamp(dt);
    let utc_offset = current_utc_offset();

    datetime.to_offset(utc_offset)
}

pub fn date_from(dt: i64) -> String {
    let datetime = datetime_from(dt);
    datetime.date().to_string()
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
    pub fn day_of_week(&self) -> String {
        let date_time = datetime_from(self.dt);
        let today = OffsetDateTime::now_local();
        let today_date = today.date();
        let date = date_time.date();
        if today_date.year() == date.year()
        && today_date.month() == date.month()
        && today_date.day() == date.day() {
            String::from("Today")
        } else {
            date_time.weekday().to_string()
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
        
        let data: Option<WeatherData> = serde_json::from_str(&text)
            .unwrap();
            //.unwrap_or(None);

        if let Some(mut data) = data {
            data.units = Some(units);
            Some(data)
        } else {
            None
        }
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
