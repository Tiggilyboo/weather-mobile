use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::env::current_dir;

use gtk::prelude::*;
use gtk::{
    ApplicationWindow,
    ActionBar,
    Label,
    Image,
};
use flume::{
    Receiver,
    unbounded,
};
use super::preferences::WeatherPreferences;
use super::api::{
    weather::*,
    location::*,
    units::Units,
};
use super::rpc::WeatherUpdate;

pub struct WeatherApplication {
    location: Label,
    temperature: Label,
    feels_like: Label,
    current_image: Image,
}

pub fn load_ui(weather_app: RefCell<WeatherApplication>, preferences: WeatherPreferences) {
    let (sender, receiver) = unbounded();
    spawn_local_handler(weather_app, receiver);

    let main_ctx = gtk::glib::MainContext::default();
    let future = async move {
        let data: WeatherData = get_weather_data(
           Units::Metric, 
           preferences.lat, 
           preferences.lon,
        ).await;

        sender.send_async(WeatherUpdate::Data(data)).await.unwrap();
        sender.send_async(WeatherUpdate::Location(preferences.location)).await.unwrap();
    };
    main_ctx.spawn_local(future);
}

fn spawn_local_handler(weather_app: RefCell<WeatherApplication>, receiver: Receiver<WeatherUpdate>) {
    let main_ctx = gtk::glib::MainContext::default();
    let future = async move {
        while let Ok(item) = receiver.recv_async().await {
            if let mut app = weather_app.borrow_mut() {
                app.update(item);
            }
        }
    };
    main_ctx.spawn_local(future);
}

impl WeatherApplication {
    pub fn new(window: &ApplicationWindow) -> WeatherApplication {
        let temperature = Label::new(None);
        let feels_like = Label::new(None);
        let location = Label::new(None);
        let action_bar = ActionBar::new();
        action_bar.set_center_widget(Some(&location));

        let current_image = Image::new();
        let cbox = gtk::CenterBox::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        hbox.append(&current_image);
        hbox.append(&temperature);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        vbox.append(&hbox);
        vbox.append(&feels_like);
        vbox.append(&location);
        vbox.append(&action_bar);

        cbox.set_center_widget(Some(&vbox));

        window.set_child(Some(&cbox));

        let wa = WeatherApplication {
            temperature,
            location,
            feels_like,
            current_image,
        };

        wa
    }

    pub fn update(&mut self, update: WeatherUpdate) {
        println!("Got update from sender");
        match update {
            WeatherUpdate::Data(data) => self.update_weather(&data),
            WeatherUpdate::Location(location) => self.update_location(&location),
        }
    }

    fn update_weather(&mut self, weather: &WeatherData) {
        self.temperature.set_text(&format!("{}", weather.display_temperature(weather.current.temp)));
        self.feels_like.set_text(&format!("Feels like: {}", weather.display_temperature(weather.current.feels_like)));
        
        let image_path = Self::current_image_path(Some(weather));
        self.current_image.set_from_file(image_path);
    }

    fn current_image_path(weather: Option<&WeatherData>) -> PathBuf {
        let pwd = current_dir().unwrap();
        let path = if weather.is_some() && weather.unwrap().current.status.len() == 0 {
            let icon = weather.unwrap().current.status[0].icon.to_string();
            format!("{}/icons/{}.png", pwd.display(), &icon)
        } else {
            format!("{}/icons/unknown.png", pwd.display())
        };

        Path::new(&path).to_path_buf()
    }

    fn update_location(&mut self, location: &str) {
        self.location.set_text(location);
    }
}
