extern crate gtk;
extern crate flume;
extern crate serde;
extern crate serde_json;
extern crate time;

mod api;
mod ui;
mod preferences;
mod rpc;

use preferences::WeatherPreferences;
use ui::WeatherApplication;
use flume::unbounded;
use std::sync::{Arc, Mutex};

use gtk::ApplicationWindow;
use gtk::Application;
use gtk::prelude::*;
use gtk::glib::MainContext;

fn initialise_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Weather"));
    window.set_show_menubar(false);
    window.set_icon_name(Some("weather-few-clouds"));

    let weather_app = WeatherApplication::new(&window);
    let weather_prefs = WeatherPreferences::from_config();
    let weather_app = Arc::new(Mutex::new(weather_app));
    let mutex = Arc::downgrade(&weather_app);
    let (sender, receiver) = unbounded();

    if let Ok(mut app) = mutex.upgrade().unwrap().try_lock() {
        app.load(weather_prefs, sender, mutex);
    } else {
        panic!("Unable to load weather application");
    }

    let main_ctx = MainContext::default();
    let future = async move {
        while let Ok(item) = receiver.recv_async().await {
            match weather_app.try_lock() {
                Ok(mut app) => {
                    if app.is_active() {
                        app.update(item);
                    } else {
                        println!("Done, not active");
                        return;
                    }
                },
                Err(err) => println!("{}", err),
            }
        }
        println!("Done");
    };
    main_ctx.spawn_local(future);
    window.show();
}

fn main() {
    let app = Application::new(
        Some("com.github.tiggilyboo.weather"),
        Default::default(),
    );

    app.connect_activate(initialise_ui);
    app.run();
}
