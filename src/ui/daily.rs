use gtk::prelude::*;
use gtk::{
    Image,
    Label,
};
use crate::api::weather::DailyWeather;
use crate::api::units::Units;
use super::icon_path;
use crate::ui::hourly::{
    build_precipitation_component,
    RAIN_ICON,
    SNOW_ICON,
};

pub struct DayView {
    container: gtk::Box,
}

impl DayView {
    pub fn from_daily_weather(data: &DailyWeather, units: &Units) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let icon_path = if data.status.len() > 0 {
            icon_path(Some(data.status[0].icon.clone()))
        } else {
            icon_path(None)
        };

        let date = Label::new(None);
        date.set_markup(&format!("<b>{}</b>", &data.day_of_week()));
        container.append(&date);

        let status = Image::from_file(icon_path);
        status.set_icon_size(gtk::IconSize::Large);
        container.append(&status);

        let temp = Label::new(None);
        temp.set_markup(
            &format!("<b>{}</b>", units.temperature_value(data.temp.day)));
        let feels_like = Label::new(None);
        feels_like.set_markup(
            &format!("<small>Feels like:</small> {}", units.temperature_value(data.feels_like.day)));
        container.append(&temp);
        container.append(&feels_like);

        if data.pop > 0.00 {
            let precipitation = gtk::Box::new(gtk::Orientation::Horizontal, 5);

            let pop = Label::new(Some(&format!("{:.0}% chance", data.pop * 100.00)));
            precipitation.append(&pop);

            if let Some(rain) = build_precipitation_component(RAIN_ICON, data.rain, units) {
                precipitation.append(&rain);
            }
            if let Some(snow) = build_precipitation_component(SNOW_ICON, data.snow, units) {
                precipitation.append(&snow);
            }
            if data.snow.is_none() && data.rain.is_none() {
                precipitation.append(&Label::new(Some("of precipitation")));
            }
            container.append(&precipitation);
        }
        
        let mut details = String::new();
        details += &format!(
"<b>Temperatures</b>
  Night: {}\t({})
  Evening: {}\t({})
  Morning: {}\t({})

",
            units.temperature_value(data.temp.night), units.temperature_value(data.feels_like.night),
            units.temperature_value(data.temp.eve), units.temperature_value(data.feels_like.eve),
            units.temperature_value(data.temp.morn), units.temperature_value(data.feels_like.morn),
        );
        
        details += &format!(
"<b>Wind:</b> {} at {}º
", units.speed_value(data.wind_speed), data.wind_deg);

        let details_label = Label::new(None);
        details_label.set_markup(&details);
        container.append(&details_label);
        
        let sun_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        if let Some(sunrise) = data.sunrise() {
            let sunrise_img = Image::from_icon_name(Some("daytime-sunrise-symbolic"));
            let sunrise = Label::new(Some(&sunrise));
            sun_box.append(&sunrise_img);
            sun_box.append(&sunrise);
        }
        if let Some(sunset) = data.sunset() {
            let sunset_img = Image::from_icon_name(Some("daytime-sunset-symbolic"));
            let sunset = Label::new(Some(&sunset));
            sun_box.append(&sunset_img);
            sun_box.append(&sunset);
        }
        container.append(&sun_box);

        Self {
            container, 
        }
    }
}

pub struct DailyView {
    pub container: gtk::ScrolledWindow,
    pub views: Vec<DayView>,
    contents: gtk::Box,
}

impl DailyView {
    pub fn new() -> Self {
        let contents = gtk::Box::new(gtk::Orientation::Horizontal, 20);

        let scroller = gtk::ScrolledWindow::new();
        scroller.set_child(Some(&contents));
        scroller.set_propagate_natural_height(true);
        scroller.set_propagate_natural_width(true);
        scroller.set_kinetic_scrolling(true);
        
        Self {
            container: scroller,
            contents,
            views: Vec::new(),
        }
    }

    pub fn populate(&mut self, daily_data: Vec<DailyWeather>, units: &Units) {
        for view in self.views.iter() {
            self.contents.remove(&view.container);
        }
        self.views.clear();

        for data in daily_data.iter() {
            let view = DayView::from_daily_weather(data, units);
            self.contents.append(&view.container);
            self.views.push(view);
        }
    }

    pub fn set_visible(&self, visible: bool) {
        self.container.set_visible(visible);
    }
}
