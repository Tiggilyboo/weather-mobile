use gtk::prelude::*;
use gtk::{
    Image,
    Label,
    ListView,
};
use gtk::gio::prelude::*;
use gtk::glib::prelude::*;
use gtk::gio::{ListModel,ListStore};
use gtk::glib::{Type, VariantDict, Object};
use crate::api::weather::{
    DailyWeather,
    TimeStamped,
};

const DAILY_WEATHER_COL_COUNT: usize = 20;

fn daily_weather_to_store(data: Vec<DailyWeather>) -> ListStore {
    let store = ListStore::new(gtk::glib::types::Type::Object);
    for d in data.iter() {
        let values: [(&str, &dyn ToValue); DAILY_WEATHER_COL_COUNT] = [
            ("date", &d.date()),
            ("sunrise", &d.sunrise().unwrap()),
            ("sunset", &d.sunset().unwrap()),
            ("day" , &d.temp.day),
            ("night" , &d.temp.night),
            ("eve" , &d.temp.eve),
            ("morn" , &d.temp.morn),
            ("day" , &d.feels_like.day),
            ("night" , &d.feels_like.night),
            ("eve", &d.feels_like.eve),
            ("morn" , &d.feels_like.morn),
            ("pressure", &d.pressure),
            ("humidity", &d.humidity),
            ("dew_point", &d.dew_point),
            ("uvi", &d.uvi),
            ("clouds", &d.clouds),
            ("wind_speed", &d.wind_speed),
            ("wind_deg" , &d.wind_deg),
            ("icon", &d.status[0].icon),
            ("pop", &d.pop),
        ];

        if let Ok(obj) = Object::with_type(Type::Object, &values) {
            store.append(&obj);
        }
    }

    store
}

fn setup_daily_list_item(_: &gtk::SignalListItemFactory, list_item: &gtk::ListItem) {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let image = gtk::Image::new();
    image.set_icon_size(gtk::IconSize::Large);
    let label = Label::new(Some("Test!"));

    container.append(&image);
    container.append(&label);
    list_item.set_child(Some(&container));

    println!("LIST ITEM!");
}

fn bind_daily_list_item(_: &gtk::SignalListItemFactory, list_item: &gtk::ListItem) {
    let container = list_item.get_child().unwrap();
    let image = container.get_first_child().unwrap()
        .downcast::<Image>().unwrap();
    let label = container
        .get_next_sibling().unwrap()
        .downcast::<Label>().unwrap();
    let item = list_item.get_item().unwrap();

    label.set_label("Testing!");

    println!("BIND: {:?}", item);
}

fn build_daily_list(data: Vec<DailyWeather>) -> gtk::ListView {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(setup_daily_list_item);
    factory.connect_bind(bind_daily_list_item);

    let model = daily_weather_to_store(data);
    let selection = gtk::SingleSelection::new(Some(&model));
    let list = ListView::new(Some(&selection), Some(&factory));
                                        
    list
}

pub struct DailyView {
    pub container: gtk::Box,
    pub list: Option<ListView>,
}

impl DailyView {
    pub fn new() -> Self {
        Self {
            container: gtk::Box::new(gtk::Orientation::Horizontal, 10),
            list: None,
        }
    }

    pub fn populate(&mut self, daily_data: Vec<DailyWeather>) {
        if let Some(list) = &self.list {
            self.container.remove(list);
        }
        let daily_list = build_daily_list(daily_data);
        self.container.append(&daily_list);
        self.list = Some(daily_list);
    }
}
