extern crate gtk;
extern crate flume;
extern crate serde;
extern crate serde_json;

mod api;
mod ui;
mod preferences;
mod rpc;

use api::{
    units::Units,
    weather::WeatherData,
    location::LocationPoint,
};
use preferences::{
    WeatherPreferences,
    load_preferences,
};
use ui::{
    WeatherApplication,
    load_ui,
};
use rpc::WeatherUpdate;
use std::cell::RefCell;

use gtk::ApplicationWindow;
use gtk::Application;
use gtk::prelude::*;

fn initialise_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Weather"));
    window.set_show_menubar(false);

    let weather_app = WeatherApplication::new(&window);

    let prefs = load_preferences();
    load_ui(RefCell::new(weather_app), prefs);

    window.show();
}

fn main() {

    let app = Application::new(
        Some("com.github.tiggilyboo.weather"),
        Default::default(),
    ).expect("Initialisation failed");

    app.connect_activate(initialise_ui);
    app.run(&std::env::args().collect::<Vec<_>>());
}
