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
};
use ui::{
    WeatherApplication,
};
use rpc::WeatherUpdate;
use std::cell::RefCell;

use flume::{
    Receiver,
    Sender,
    unbounded,
};
use gtk::ApplicationWindow;

use gtk::prelude::*;
use gtk::gio::prelude::*;
use gtk::Application;

fn initialise_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Weather"));

    let weather_app = WeatherApplication::new(&window);
    let (tx, rx) = unbounded();

    spawn_local_handler(RefCell::new(weather_app), rx); 
    let prefs = load_preferences();
    start_communication_thread(prefs, tx);
    window.show();
}

fn load_preferences() -> WeatherPreferences {
    if let Some(prefs) = WeatherPreferences::from_config() {
        prefs
    } else {
        if let Some(location_data) = api::location::get_location_data("Ruinerwold") {
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

fn spawn_local_handler(weather_app: RefCell<WeatherApplication>, receiver: Receiver<WeatherUpdate>) {
    let main_ctx = gtk::glib::MainContext::default();
    let future = async move {
        while let Some(item) = receiver.try_recv().ok() {
            if let mut app = weather_app.borrow_mut() {
                app.update(item);
            }
        }
    };
    main_ctx.spawn_local(future);
}

fn start_communication_thread(preferences: WeatherPreferences, sender: Sender<WeatherUpdate>) {
   let data: WeatherData = api::weather::get_weather_data(
       Units::Metric, 
       preferences.lat, 
       preferences.lon);
   let _ = sender.send(WeatherUpdate::Data(data));
   let _ = sender.send(WeatherUpdate::Location(preferences.location));
}

fn main() {

    let app = Application::new(
        Some("com.github.tiggilyboo.weather"),
        Default::default(),
    ).expect("Initialisation failed");

    app.connect_activate(initialise_ui);
    app.run(&std::env::args().collect::<Vec<_>>());
}
