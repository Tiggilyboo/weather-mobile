use std::sync::{Mutex, Weak};
use std::path::{Path, PathBuf};
use std::env::current_dir;
use core::future::Future;

use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    ActionBar,
    Label,
    EditableLabel,
    Picture,
    Entry,
    Button,
    ComboBoxText,
    ListStore,
};
use gtk::glib::value::*;
use gtk::glib::{Value, TypedValue};
use flume::{
    Sender,
};
use super::preferences::WeatherPreferences;
use super::api::{
    weather::*,
    location::*,
    units::Units,
};
use super::rpc::WeatherUpdate;

pub struct WeatherApplication {
    application: Weak<Application>,
    location: EditableLabel,
    location_search: Entry,
    location_search_button: Button,
    location_results: ComboBoxText,
    temperature: Label,
    feels_like: Label,
    current_picture: Picture,
    active: bool,
    sender: Option<Sender<WeatherUpdate>>,
    mutex: Option<Weak<Mutex<Self>>>,
}

impl WeatherApplication {
    pub fn new(application: Weak<Application>, window: &ApplicationWindow) -> Self {
        let temperature = Label::new(None);
        let feels_like = Label::new(None);
        let location = EditableLabel::new("");
        let location_search = Entry::new();
        let location_search_button = Button::with_label("Search");
        let location_results = ComboBoxText::new();
        location_results.set_visible(false);
        location_results.set_id_column(0);

        let location_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        location_search.set_placeholder_text(Some("Search your location..."));
        location_box.append(&location);
        location_box.append(&location_search);
        location_box.append(&location_results);
        location_box.append(&location_search_button);

        let action_bar = ActionBar::new();
        action_bar.set_center_widget(Some(&location_box));

        let current_picture = Picture::new();
        let cbox = gtk::CenterBox::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);

        hbox.append(&current_picture);
        hbox.append(&temperature);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        vbox.append(&hbox);
        vbox.append(&feels_like);
        vbox.append(&action_bar);

        cbox.set_center_widget(Some(&vbox));

        window.set_child(Some(&cbox));

        let wa = WeatherApplication {
            application,
            temperature,
            location,
            location_search,
            location_search_button,
            location_results,
            feels_like,
            current_picture,
            active: true,
            sender: None, 
            mutex: None,
        };
    
        wa
    }

    fn spawn_local<Fs: 'static + Future<Output = ()>>(&self, sender_future: Fs) {
        gtk::glib::MainContext::default().spawn_local(sender_future);
    }

    fn get_sender(&self) -> Sender<WeatherUpdate> {
        self.sender.clone()
            .expect("Unable to find application sender")
    }

    pub fn get_mutex(&self) -> Weak<Mutex<Self>> {
        self.mutex.clone().unwrap()
    }
     
    pub fn load(&mut self,
        preferences: Option<WeatherPreferences>,
        sender: Sender<WeatherUpdate>,
        mutex: Weak<Mutex<Self>>) {

        self.sender = Some(sender);

        // Bind signals
        let mutex_location = mutex.clone();
        self.location.connect_property_editing_notify(move |l| {
            if !l.get_editing() {
                return;
            }
            if let Ok(app) = mutex_location.upgrade().unwrap().try_lock() {
                app.get_sender().send(WeatherUpdate::Location(None))
                    .expect("Unable to send WeatherUpdate(None) for location");
            }
        });
        
        let mutex_location_search = mutex.clone();
        self.location_search_button.connect_clicked(move |button| { 
            if let Ok(app) = mutex_location_search.upgrade().unwrap().try_lock() {
                let button_label = button.get_label().unwrap().clone();
                match button_label.as_str() {
                    "Search" => {
                        if let Some(search_query) = app.location_search.get_text() {
                            if search_query.len() == 0 {
                                return;
                            }
                            let search_query: &str = &search_query;
                            app.get_sender().send(WeatherUpdate::SearchLocations(search_query.to_string()))
                                .expect("Unable to send WeatherUpdate::Location(None) for Search");
                        } else {
                            println!("Unable to lock mutex_location");
                        }
                    },
                    "Cancel" => {
                        app.get_sender().send(WeatherUpdate::Location(None))
                            .expect("Unable to send WeatherUpdate::Location(None) for Cancel"); 
                    }, 
                    _ => panic!(format!("Unhandled location action: {}", button_label))
                }
            }
        });

        let mutex_combo = mutex.clone();
        self.location_results.connect_changed(move |combo| {
            if let Some(active_iter) = combo.get_active_iter() {
                if let Some(model) = combo.get_model() {
                    let location = model.get_value(&active_iter, 0).get::<String>()
                        .expect("location from model at col 0 is String")
                        .unwrap();
                    let lat = model.get_value(&active_iter, 1).get::<f32>()
                        .expect("lat from model at col 1 is F32")
                        .unwrap();
                    let lon = model.get_value(&active_iter, 2).get::<f32>()
                        .expect("lon from model at col 2 is F32")
                        .unwrap();

                    let interest = LocationPoint {
                        location,
                        lat,
                        lon,
                    };
                    if let Ok(app) = mutex_combo.upgrade().unwrap().try_lock() {
                        app.request_weather(interest);
                    }
                }
            }
        });
        self.mutex = Some(mutex);

        if let Some(preferences) = preferences {
            self.request_weather(LocationPoint {
                location: preferences.location,
                lat: preferences.lat,
                lon: preferences.lon,
            });
        } else {
            // No preferences set! Set ui state as no-location
            let sender = self.get_sender();
            self.spawn_local(async move {
                if let Err(err) = sender.send_async(WeatherUpdate::Location(None)).await {
                    println!("Unable to request weather: {}", err);
                }
            });
        }
    }

    fn request_weather(&self, interest: LocationPoint) {
        let mutex = self.get_mutex();

        self.spawn_local(async move {
            if let Ok(app) = mutex.upgrade().unwrap().try_lock() {
                let sender = app.get_sender();
                let data: WeatherData = get_weather_data(
                   Units::Metric, 
                   interest.lat, 
                   interest.lon,
                ).await;

                sender.send_async(WeatherUpdate::Data(data)).await.unwrap();
                sender.send_async(WeatherUpdate::Location(Some(interest.location))).await.unwrap();
            }
        });
    }

    pub fn update(&mut self, update: WeatherUpdate) {
        match update {
            WeatherUpdate::Data(data) => self.update_weather(&data),
            WeatherUpdate::Location(location) => self.update_location(location),
            WeatherUpdate::Exit => self.handle_quit(),
            WeatherUpdate::SearchLocations(query) => self.search_location(query),
            WeatherUpdate::SetLocations(locations) => self.update_location_results(locations),
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }

    fn update_weather(&mut self, weather: &WeatherData) {
        self.temperature.set_markup(&format!("<big>{}</big>", weather.display_temperature(weather.current.temp)));
        self.feels_like.set_markup(&format!("<big>Feels like: {}</big>", weather.display_temperature(weather.current.feels_like)));
        
        let picture_path = Self::current_picture_path(Some(weather));
        self.current_picture.set_filename(Some(picture_path.to_str().unwrap()));
    }

    fn current_picture_path(weather: Option<&WeatherData>) -> PathBuf {
        let pwd = current_dir().unwrap();
        let path = if weather.is_some() && weather.unwrap().current.status.len() > 0 {
            println!("weather status: {}", weather.unwrap().current.status[0].icon);
            let icon = weather.unwrap().current.status[0].icon.to_string();
            format!("{}/icons/{}.png", pwd.display(), &icon)
        } else {
            format!("{}/icons/unknown.png", pwd.display())
        };

        Path::new(&path).to_path_buf()
    }

    fn search_location(&self, search_query: String) {
        let search_query = search_query.clone();
        let mutex = self.get_mutex();

        self.spawn_local(async move {
            match mutex.upgrade().unwrap().try_lock() {
                Ok(app) => {
                    app.location.set_visible(false);
                    app.location_search.set_visible(false); 
                    app.location_search_button.set_visible(false);
                    if let Some(locations) = search_locations(&search_query).await {
                        let sender = app.get_sender();
                        let sent = sender.send_async(WeatherUpdate::SetLocations(locations)).await;
                        if sent.is_err() {
                            println!("Unable to send WeatherUpdate::SetLocations");
                        }
                    } else {
                        // TODO: some feedback
                        println!("Unable to retreive results from locations");
                    }
                }, 
                Err(err) => println!("search_location err: {}", err),
            }
        });
    }

    fn locations_to_store(locations: Vec<LocationPoint>) -> ListStore {
        let col_types: [gtk::glib::Type; 3] = [
            gtk::glib::Type::String, 
            gtk::glib::Type::F32,
            gtk::glib::Type::F32,
        ];
        let model = ListStore::new(&col_types);
        let col_indices: [u32; 3] = [0, 1, 2];

        for l in locations.iter() {
            let values: [&dyn ToValue; 3] = [&l.location, &l.lat, &l.lon];
            model.set(&model.append(), &col_indices, &values);
        }

        model
    }

    fn update_location_results(&mut self, location_results: Vec<LocationPoint>) {
        let results_count = location_results.len();
        let first_result = if results_count == 1 {
            Some(location_results[0].clone())
        } else {
            None
        };
        let list_model = Self::locations_to_store(location_results);
        self.location_results.set_model(Some(&list_model));
        self.location_results.set_visible(true);

        match results_count {
            1 => {
                if let Some(first) = first_result {
                    // Force change trigger
                    self.location_results.set_active_id(Some(&first.location));
                    self.request_weather(first);
                } 
            }, 
            _ => {
                self.location_results.popup();
            },
        }
    }

    fn update_location(&mut self, location: Option<String>) {
        if let Some(location) = location {
            self.location.set_text(&location);
            self.location.set_visible(true);
            self.location_search.set_visible(false);
            self.location_results.set_visible(false);
            self.location_search_button.set_visible(false);
        } else {
            self.location.set_visible(false);
            self.location_search.set_visible(true);    
            self.location_search.set_text("");
            self.location_search_button.set_visible(true);
            self.location_search_button.set_label("Search");
        }
    }
    
    fn handle_quit(&mut self) {
        self.active = false;
        if let Some(app) = self.application.upgrade() {
            app.quit();
        }
    }
}
