use dirs::config_dir;
use std::path::PathBuf;
use std::fs::File;
use serde::{
    Serialize,
    Deserialize,
};
use super::api::location::get_location_data;

#[derive(Serialize, Deserialize)]
pub struct WeatherPreferences {
    pub location: String,
    pub lat: f32,
    pub lon: f32,
}

const WEATHER_CONFIG_FILE: &str = "weather.json";

pub fn load_preferences() -> WeatherPreferences {
    if let Some(prefs) = WeatherPreferences::from_config() {
        prefs
    } else {
        if let Some(location_data) = get_location_data("Ruinerwold") {
            let new_prefs = WeatherPreferences::new(
                location_data.lat, 
                location_data.lon, 
                location_data.location);
            new_prefs.save_config();
            new_prefs
        } else {
            panic!("Unable to get location data")
        }   

    }
}

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

    pub fn new(lat: f32, lon: f32, location: String) -> WeatherPreferences {
        Self {
            lat,
            lon,
            location,
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
