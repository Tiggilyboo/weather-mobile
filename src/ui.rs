use gtk::{
    ApplicationWindow,
    Label,
    Image,
};
use gtk::prelude::*;
use super::api::{
    weather::WeatherData,
};
use super::rpc::WeatherUpdate;
use std::path::Path;
use std::env::current_dir;

pub struct WeatherApplication {
    location: Label,
    temperature: Label,
    feels_like: Label,
    current_image: Option<Image>,
}

impl WeatherApplication {
    pub fn new(window: &ApplicationWindow) -> WeatherApplication {
        let temperature = Label::new(None);
        let location = Label::new(None);
        let feels_like = Label::new(None);
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10); 

        vbox.add(&temperature);
        vbox.add(&feels_like);
        vbox.add(&location);

        window.add(&vbox);

        let wa = WeatherApplication {
            temperature,
            location,
            feels_like,
            current_image: None,
        };

        wa
    }

    pub fn update(&mut self, update: WeatherUpdate) {
        match update {
            WeatherUpdate::Data(data) => self.update_weather(&data),
            WeatherUpdate::Location(location) => {
                self.location.set_text(&format!("Location: {}", location));
            },
        }
    }

    fn update_weather(&mut self, weather: &WeatherData) {
        self.temperature.set_text(&format!("{}", weather.display_temperature(weather.current.temp)));
        self.feels_like.set_text(&format!("Feels like: {}", weather.display_temperature(weather.current.feels_like)));
        
        let image = Self::load_image(Some(weather));
        self.current_image = Some(image);
    }

    fn load_image(weather: Option<&WeatherData>) -> Image {
        let pwd = current_dir().unwrap();
        let path = if weather.is_some() && weather.unwrap().current.status.len() == 0 {
            let icon = weather.unwrap().current.status[0].icon.to_string();
            format!("{}/icons/{}.png", pwd.display(), &icon)
        } else {
            format!("{}/icons/unknown.png", pwd.display())
        };

        let icon_path = Path::new(&path);
        println!("Loading {} from file for image status", icon_path.to_str().unwrap());
        
        Image::from_file(icon_path)
    }
}
