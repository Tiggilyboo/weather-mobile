mod daily;
mod alert;

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
    Expander,
    Picture,
    Image,
    Entry,
    Button,
    Switch,
    ComboBoxText,
    ListStore,
};
use flume::{
    Sender,
};
use super::preferences::WeatherPreferences;
use super::api::{
    weather::*,
    location::*,
    units::Units,
};
use alert::WeatherAlerts;
use daily::DailyView;
use super::rpc::WeatherUpdate;

pub struct WeatherApplication {
    active: bool,
    sender: Option<Sender<WeatherUpdate>>,
    mutex: Option<Weak<Mutex<Self>>>,
    location: EditableLabel,
    location_search: Entry,
    location_search_button: Button,
    location_results: ComboBoxText,
    refresh_button: Button,
    temperature: Label,
    feels_like: Label,
    current_details: Label,
    current_details_expander: Expander,
    current_picture: Picture,
    alerts: WeatherAlerts,
    daily: DailyView,
    preferences: Option<WeatherPreferences>,
}

pub fn icon_path(icon: Option<String>) -> PathBuf {
    let pwd = current_dir().unwrap();
    let path = if let Some(icon) = icon {
        format!("{}/icons/{}.png", pwd.display(), &icon)
    } else {
        format!("{}/icons/unknown.png", pwd.display())
    };
    Path::new(&path).to_path_buf()
}

fn current_picture_path(current: Option<&CurrentWeather>) -> PathBuf {
    let path = if current.is_some() && current.unwrap().status.len() > 0 {
        icon_path(Some(current.unwrap().status[0].icon.clone()))
    } else {
        icon_path(None)
    };

    Path::new(&path).to_path_buf()
}

impl WeatherApplication {
    pub fn new(application: Weak<Application>, window: &ApplicationWindow) -> Self {
        let temperature = Label::new(None);
        let feels_like = Label::new(None);
        let location = EditableLabel::new("");
        location.set_visible(false);

        let location_image = Image::from_icon_name(Some("network-workgroup-symbolic"));

        let location_search = Entry::new();
        let location_search_button = Button::from_icon_name(Some("edit-find"));
        let location_results = ComboBoxText::new();
        location_results.set_visible(false);
        location_results.set_id_column(0);

        let location_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        location_search.set_placeholder_text(Some("Search your location..."));
        location_box.append(&location_image);
        location_box.append(&location);
        location_box.append(&location_search);
        location_box.append(&location_results);
        location_box.append(&location_search_button);

        let action_bar = ActionBar::new();
        action_bar.set_center_widget(Some(&location_box));
        
        let refresh_button = Button::from_icon_name(Some("view-refresh"));
        refresh_button.set_visible(false);
        action_bar.pack_end(&refresh_button);

        let current_picture = Picture::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        hbox.append(&current_picture);
        hbox.append(&temperature);

        let chbox = gtk::CenterBox::new();
        chbox.set_center_widget(Some(&hbox));

        let current_details = Label::new(None);
        let current_details_expander = Expander::new(Some("Current"));
        current_details_expander.set_child(Some(&current_details));
        current_details_expander.set_visible(false);

        let alerts_container = gtk::CenterBox::new();
        let alerts = WeatherAlerts::new(None);
        alerts_container.set_center_widget(Some(&alerts.container));

        let daily = DailyView::new();
        daily.set_visible(false);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        vbox.append(&action_bar);
        vbox.append(&alerts_container);
        vbox.append(&chbox);
        vbox.append(&feels_like);
        vbox.append(&current_details_expander);
        vbox.append(&daily.container);

        window.set_child(Some(&vbox));

        let wa = WeatherApplication {
            temperature,
            location,
            location_search,
            location_search_button,
            location_results,
            refresh_button,
            feels_like,
            current_picture,
            current_details,
            current_details_expander,
            alerts,
            daily,
            active: true,
            sender: None, 
            mutex: None,
            preferences: None,
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
        self.location_search_button.connect_clicked(move |_| { 
            if let Ok(app) = mutex_location_search.upgrade().unwrap().try_lock() {
                if let Some(search_query) = app.location_search.get_text() {
                    if search_query.len() == 0 {
                        return;
                    }
                    let search_query: &str = &search_query;
                    app.get_sender().send(WeatherUpdate::SearchLocations(search_query.to_string()))
                        .expect("Unable to send WeatherUpdate::SearchLocations(None) for Search");
                } else {
                    println!("Unable to lock mutex_location");
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
                    let lat = model.get_value(&active_iter, 1).get::<f64>()
                        .expect("lat from model at col 1 is F64")
                        .unwrap();
                    let lon = model.get_value(&active_iter, 2).get::<f64>()
                        .expect("lon from model at col 2 is F64")
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

        let mutex_refresh = mutex.clone();
        self.refresh_button.connect_clicked(move |_| {
            if let Ok(app) = mutex_refresh.upgrade().unwrap().try_lock() {
                app.refresh_weather();
            }
        });

        // must be set before request_weather
        self.mutex = Some(mutex);

        // Load current weather if preferences set
        if let Some(preferences) = &preferences {
            self.request_weather(LocationPoint {
                location: preferences.location.clone(),
                lat: preferences.lat,
                lon: preferences.lon,
            });
        } else {
            // No preferences set! Set ui state as no-location
            if let Ok(app) = self.get_mutex().clone().upgrade().unwrap().try_lock() {
                if let Err(_) = app.get_sender().send(WeatherUpdate::Location(None)) {
                    println!("Unable to reset location when preferences were not set");
                }
            }
        }

        self.preferences = preferences;
    }

    fn refresh_weather(&self) {
        if let Some(prefs) = &self.preferences {
            self.request_weather(LocationPoint {
                location: prefs.location.clone(),
                lat: prefs.lat,
                lon: prefs.lon,
            });
        }
    }

    fn request_weather(&self, interest: LocationPoint) {
        let mutex = self.get_mutex().clone();

        self.spawn_local(async move {
            if let Ok(app) = mutex.upgrade().unwrap().try_lock() {
                let sender = app.get_sender();

                let new_prefs = WeatherPreferences {
                    location: interest.location,
                    lat: interest.lat,
                    lon: interest.lon,
                    units: app.get_units(),
                };
                let data = get_weather_data(
                   app.get_units(),
                   interest.lat, 
                   interest.lon,
                ).await;

                sender.send_async(WeatherUpdate::Data(data)).await.unwrap();
                sender.send_async(WeatherUpdate::Location(Some(new_prefs.location.clone()))).await.unwrap();
                if let Err(err) = sender.send_async(WeatherUpdate::SavePreferences(new_prefs)).await {
                    println!("Unable to save preferences: {}", err);
                }
            }
        });
    }

    pub fn update(&mut self, update: WeatherUpdate) {
        match update {
            WeatherUpdate::Data(data) => self.update_weather(data),
            WeatherUpdate::Location(location) => self.update_location(location),
            WeatherUpdate::SearchLocations(query) => self.search_location(query),
            WeatherUpdate::SetLocations(locations) => self.update_location_results(locations),
            WeatherUpdate::SavePreferences(preferences) => self.save_preferences(&preferences),
        }
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }

    fn update_daily_weather(&mut self, daily: Option<Vec<DailyWeather>>) {
        if let Some(daily) = daily {
            self.daily.populate(daily, &self.get_units());
            self.daily.set_visible(true);
        } else {
            self.daily.populate(Vec::new(), &self.get_units());
            self.daily.set_visible(false);
        }
    }

    fn update_current_image(&mut self, current: Option<CurrentWeather>) {
        let picture_path = current_picture_path(current.as_ref());
        self.current_picture.set_filename(Some(picture_path.to_str().unwrap()));
    }

    fn update_current_weather(&mut self, current: Option<CurrentWeather>) {
        if let Some(current) = current {
            let units = self.get_units();
            self.temperature.set_markup(&format!("<big>{}</big>", units.temperature_value(current.temp)));
            self.feels_like.set_markup(&format!("<big>Feels like: {}</big>", units.temperature_value(current.feels_like)));
            self.current_details.set_markup(&format!("
<b>At</b> {}
Pressure: {}
Humidity: {}
UV Index: {}
Visibility: {}
Wind Speed: {}
Precipitation: {}%
            ", 
            current.time("%T"), 
            current.pressure, 
            current.humidity,
            current.uvi,
            current.visibility.unwrap_or(0),
            units.speed_value(current.wind_speed),
            current.pop * 100.00));
            self.current_details_expander.set_visible(true);
            self.update_current_image(Some(current));
            
        } else {
            self.temperature.set_markup("<big>Invalid Data</big>");
            self.feels_like.set_markup("Please try another city name!");

            self.current_details_expander.set_visible(false);
            self.update_current_image(None);
        };
        
    }
    
    fn update_alerts(&mut self, weather_alerts: Option<Vec<WeatherAlert>>) {
        if let Some(weather_alerts) = weather_alerts {
            self.alerts.populate(weather_alerts);
        } else {
            self.alerts.populate(Vec::new());
        }
    }

    fn update_weather(&mut self, weather: Option<WeatherData>) {
        if let Some(weather) = weather {
            let units = weather.units.expect("units");
            self.update_units(units);
            self.update_current_weather(Some(weather.current));
            self.update_daily_weather(Some(weather.daily));
            self.update_alerts(Some(weather.alerts));
        } else {
            self.update_current_weather(None);
            self.update_daily_weather(None);
            self.update_alerts(None);
        };
    }

    fn search_location(&self, search_query: String) {
        let search_query = search_query.clone();
        if search_query.len() == 0 {
            return;
        }
        
        let mutex = self.get_mutex();

        self.spawn_local(async move {
            match mutex.upgrade().unwrap().try_lock() {
                Ok(app) => {
                    app.location.set_visible(false);
                    app.location_search.set_visible(false); 
                    app.location_search_button.set_visible(false);

                    let sender = app.get_sender();
                    let locations = search_locations(&search_query).await;
                    if let Err(_) = sender.send_async(WeatherUpdate::SetLocations(locations)).await {
                        println!("Unable to send WeatherUpdate::SetLocations");
                    }
                }, 
                Err(err) => println!("search_location err: {}", err),
            }
        });
    }

    fn locations_to_store(locations: Vec<LocationPoint>) -> ListStore {
        let col_types: [gtk::glib::Type; 3] = [
            gtk::glib::Type::String, 
            gtk::glib::Type::F64,
            gtk::glib::Type::F64,
        ];
        let model = ListStore::new(&col_types);
        let col_indices: [u32; 3] = [0, 1, 2];

        for l in locations.iter() {
            let values: [&dyn ToValue; 3] = [&l.location, &l.lat, &l.lon];
            model.set(&model.append(), &col_indices, &values);
        }

        model
    }

    fn update_location_results(&mut self, location_results: Option<Vec<LocationPoint>>) {
        if let Some(location_results) = location_results {
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
        } else {
            if let Err(err) = self.get_sender().send(WeatherUpdate::Location(None)) {
                println!("Unable to send WeatherUpdate::Location(None): {}", err);
            }
        }
    }

    fn update_location(&mut self, location: Option<String>) {
        if let Some(location) = location {
            self.location.set_visible(true);
            self.location_search.set_visible(false);
            self.location_results.set_visible(false);
            self.location_search_button.set_visible(false);
            self.refresh_button.set_visible(true);
            self.daily.set_visible(true);
            self.location.set_text(&location);
        } else {
            self.location.set_text("");
            self.location.set_visible(false);
            self.location_search.set_visible(true);    
            self.location_search_button.set_visible(true);
            self.refresh_button.set_visible(false);
            self.daily.set_visible(false);
            self.location_search.set_text("");
            self.update_current_weather(None);
            self.update_daily_weather(None);
        }
    }

    fn save_preferences(&self, preferences: &WeatherPreferences) {
        preferences.save_config();
    }

    fn update_units(&mut self, units: Units) {
        if let Some(prefs) = &mut self.preferences {
            prefs.units = units;
        }
    }

    fn get_units(&self) -> Units {
        if let Some(prefs) = &self.preferences {
            match prefs.units {
                Units::Metric => Units::Metric,
                Units::Imperial => Units::Imperial,
            }
        } else {
            Units::Metric
        }
    }
}
