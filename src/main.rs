extern crate gtk;
extern crate glib;
extern crate flume;
extern crate serde;
extern crate serde_json;

mod api;
mod ui;
use api::{
    units::Units,
    WeatherData,
};
use ui::{
    WeatherApplication,
};
use std::cell::RefCell;

use flume::{
    Receiver,
    Sender,
    unbounded,
};
use gtk::ApplicationWindow;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::Application;

fn initialise_ui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Weather");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(720, 1440);

    let mut weather_app = WeatherApplication::new(&window);
    let (tx, rx) = unbounded();

    spawn_local_handler(RefCell::new(weather_app), rx); 
    start_communication_thread(tx);
    window.show_all();
}

fn spawn_local_handler(weather_app: RefCell<WeatherApplication>, receiver: Receiver<WeatherData>) {
    let main_ctx = glib::MainContext::default();
    let future = async move {
        while let Some(item) = receiver.try_recv().ok() {
            if let mut app = weather_app.borrow_mut() {
                app.update(&item);
            }
        }
    };
    main_ctx.spawn_local(future);
}

fn start_communication_thread(sender: Sender<WeatherData>) {
   let data: WeatherData = api::get_weather_data(Units::Metric, 33.441792, -94.037689);
   let _ = sender.send(data);
}

fn main() {

    let app = Application::new(
        Some("com.github.tiggilyboo.weather"),
        Default::default(),
    ).expect("Initialisation failed");

    app.connect_activate(initialise_ui);
    app.run(&std::env::args().collect::<Vec<_>>());
}
