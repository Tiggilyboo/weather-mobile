use gtk::prelude::*;
use gtk::{
    Expander,
    Image,
    Label,
};
use crate::api::weather::{
    DailyWeather,
    TimeStamped,
};
use super::icon_path;

pub struct DayView {
    container: gtk::Box,
}

impl DayView {
    pub fn from_daily_weather(data: &DailyWeather) -> Self {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        let icon_path = if data.status.len() > 0 {
            icon_path(Some(data.status[0].icon.clone()))
        } else {
            icon_path(None)
        };

        let date = Label::new(None);
        date.set_markup(&format!("<b>{}</b>\n<small>{}</small>", &data.day_of_week(), &data.date()));
        container.append(&date);

        let status = Image::from_file(icon_path);
        status.set_icon_size(gtk::IconSize::Large);
        container.append(&status);

        let temp = Label::new(None);
        temp.set_markup(
            &format!("<b>{}</b>", data.temp.day));
        let feels_like = Label::new(None);
        feels_like.set_markup(
            &format!("<small>Feels like:</small> {}", data.feels_like.day));
        container.append(&temp);
        container.append(&feels_like);

        let pop = Label::new(Some(&format!("Precipitation: {}%", data.pop * 100.00)));
        container.append(&pop);

        let mut details = String::new();
        details += &format!(
"<b>Temperatures</b>
  Night: {} ({})
  Evening: {} ({})
  Morning: {} ({})
",
            data.temp.night, data.feels_like.night,
            data.temp.eve, data.feels_like.eve,
            data.temp.morn, data.feels_like.morn,
        );
        
        details += &format!(
"<b>Wind</b>
  Speed: {}
  Direction (deg): {}
", data.wind_speed, data.wind_deg);

        let mut sun_info = String::new();
        if let Some(sunset) = data.sunset() {
            sun_info += &format!("<b>Sunset:</b> {}\n", sunset);
        }
        if let Some(sunrise) = data.sunrise() {
            sun_info += &format!("<b>Sunrise:</b> {}", sunrise); 
        }
        details += &sun_info;

        let details_label = Label::new(None);
        details_label.set_markup(&details);
        container.append(&details_label);

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

        let expander = Expander::new(Some("Week"));
        expander.set_child(Some(&contents));
        expander.set_expanded(true);

        let container = gtk::ScrolledWindow::new();
        container.set_child(Some(&expander));
        container.set_propagate_natural_height(true);
        container.set_kinetic_scrolling(true);
        
        Self {
            container,
            contents,
            views: Vec::new(),
        }
    }

    pub fn populate(&mut self, daily_data: Vec<DailyWeather>) {
        for view in self.views.iter() {
            self.contents.remove(&view.container);
        }
        self.views.clear();

        for data in daily_data.iter() {
            let view = DayView::from_daily_weather(data);
            self.contents.append(&view.container);
            self.views.push(view);
        }

        self.container.set_visible(daily_data.len() > 0);
    }
}
