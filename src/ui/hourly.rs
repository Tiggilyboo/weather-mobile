use crate::api::units::Units;
use crate::api::weather::{
    HourlyWeather,
    TimeStamped,
};
use super::icon_path;

use gtk4::prelude::*;
use gtk4::{
    Label,
    Image,
    ScrolledWindow,
};

pub struct HourlyView {
    pub container: gtk4::ScrolledWindow, 
    contents: gtk4::Box,
    hours: Vec<gtk4::Box>,
}

pub const RAIN_ICON: &str = "weather-showers-scattered";
pub const SNOW_ICON: &str = "weather-snow";
pub const GUST_ICON: &str = "weather-windy-symbolic";

pub fn build_wind_component(data: Option<f64>, units: &Units) -> Option<gtk4::Box> {
    if let Some(speed) = data {
        let speed = &units.speed_value(speed);
        let wind_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 3);
        wind_box.append(&Image::from_icon_name(Some(GUST_ICON)));
        wind_box.append(&Label::new(Some(speed)));

        Some(wind_box)
    } else {
        None
    }
}

pub fn build_precipitation_component(icon: &str, data: Option<f64>, units: &Units) -> Option<gtk4::Box> {
    if let Some(volume) = data {
        let rain_volume = &units.volume_value(volume);
        let rain_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 3);
        rain_box.append(&Image::from_icon_name(Some(icon)));
        rain_box.append(&Label::new(Some(rain_volume)));

        Some(rain_box)
    } else {
        None
    }
}

fn build_hourly_component(data: &HourlyWeather, units: &Units) -> gtk4::Box {
    let component = gtk4::Box::new(gtk4::Orientation::Vertical, 5);

    let time = Label::new(Some(&data.time("%T")));
    component.append(&time);

    let icon_path = if data.status.len() > 0 {
        icon_path(Some(data.status[0].icon.clone()))
    } else {
        icon_path(None)
    };
    let status = Image::from_file(icon_path);
    status.set_icon_size(gtk4::IconSize::Large);
    component.append(&status);

    let temperature = Label::new(Some(&units.temperature_value(data.temp)));
    component.append(&temperature);

    if let Some(data) = &data.rain {
        if let Some(rain_box) = build_precipitation_component(RAIN_ICON, data.volume, units) {
            component.append(&rain_box);
        }
    }
    if let Some(data) = &data.snow {
        if let Some(snow_box) = build_precipitation_component(SNOW_ICON, data.volume, units) {
            component.append(&snow_box);
        }
    }
    if let Some(gust_box) = build_wind_component(data.wind_gust, units) {
        component.append(&gust_box);
    }

    component
}

impl HourlyView {
    pub fn new() -> Self {
        let contents = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);

        let scroller = ScrolledWindow::new();
        scroller.set_child(Some(&contents));
        scroller.set_propagate_natural_height(true);
        scroller.set_propagate_natural_width(true);
        scroller.set_kinetic_scrolling(true);

        Self {
            container: scroller,
            contents,
            hours: Vec::new(),
        }
    }

    pub fn populate(&mut self, hourly: Vec<HourlyWeather>, units: &Units) {
        for hour in self.hours.iter() {
            self.contents.remove(hour);
        }
        self.hours.clear();

        for hour in hourly.iter() {
            let hour_component = build_hourly_component(hour, units);
            self.contents.append(&hour_component);
            self.hours.push(hour_component);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        self.container.set_visible(visible);
    }
}

