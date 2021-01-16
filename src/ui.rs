use gtk::{
    ApplicationWindow,
    Label,
    Image,
    Grid,
};
use gtk::prelude::*;
use super::api::WeatherData;
use std::path::Path;
use std::env::current_dir;
use super::api::units::Units;

pub struct WeatherApplication {
    location: Label,
    temperature: Label,
    feels_like: Label,
    current_image: Option<Image>,
    grid: Grid,
}

impl WeatherApplication {
    pub fn new(window: &ApplicationWindow) -> WeatherApplication {
        let temperature = Label::new(None);
        let location = Label::new(None);
        let feels_like = Label::new(None);
        let grid = Grid::new(); 

        grid.add(&temperature);
        grid.add(&feels_like);
        grid.add(&location);

        window.add(&grid);

        let wa = WeatherApplication {
            temperature,
            location,
            feels_like,
            grid,
            current_image: None,
        };

        wa
    } 

    pub fn update(&mut self, weather: &WeatherData) {
        self.temperature.set_text(&format!("{}", weather.display_temperature(weather.current.temp)));
        self.feels_like.set_text(&format!("Feels like: {}", weather.display_temperature(weather.current.feels_like)));
        
        let image = Self::load_image(Some(weather));
        self.grid.add(&image);
        self.current_image = Some(image);
    }

    fn load_image(weather: Option<&WeatherData>) -> Image {
        let pwd = current_dir().unwrap();
        let path = if weather.is_some() && weather.unwrap().current.status.len() == 0 {
            let icon = weather.unwrap().current.status[0].icon.to_string();
            format!("{}/icons/{}.png", pwd.display(), &icon)
        } else {
            String::from("{}/icons/unknown.png")
        };

        let icon_path = Path::new(&path);
        println!("Loading {} from file for image status", icon_path.to_str().unwrap());
        
        Image::from_file(icon_path)
    }
}
