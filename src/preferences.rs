use dirs::config_dir;
use std::path::PathBuf;
use std::fs::File;
use serde::{
    Serialize,
    Deserialize,
};
use super::api::units::Units;

#[derive(Serialize, Deserialize)]
pub struct WeatherPreferences {
    pub location: String,
    pub lat: f64,
    pub lon: f64,
    pub units: Units,
}

const WEATHER_CONFIG_FILE: &str = "weather.json";

fn config_path() -> PathBuf {
    if let Some(mut dir) = config_dir() {
        dir.push(WEATHER_CONFIG_FILE);
        let path = dir.as_path();

        path.to_path_buf()
    } else {
        panic!("Unable to resolve configuration path")
    }
}

impl WeatherPreferences {
    pub fn from_config() -> Option<WeatherPreferences> {
        let path = config_path();
        if path.exists() {
            let file = File::open(path)
                .expect("Unable to read config file");
            let file_value = serde_json::from_reader(file)
                .expect("File contained incorrect JSON format");
            let config_file: WeatherPreferences = serde_json::from_value(file_value)
                .expect("Config file in the wrong format");

            Some(config_file)
        } else {
            None 
        }
    }

    pub fn new(lat: f64, lon: f64, location: String, units: Units) -> WeatherPreferences {
        Self {
            lat,
            lon,
            location,
            units,
        }
    }

    pub fn save_config(&self) {
        let path = config_path();
        let file = File::create(path)
            .expect("Unable to create configuration file");
        serde_json::to_writer_pretty(file, &self)
            .expect("Unable to write to configuration file");
    }
}
