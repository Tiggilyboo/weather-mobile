use gtk::prelude::*;
use gtk::{
    Label,
    Picture,
};

pub struct Day {
   when: Label,
   temp: Label,
   feels_like: Label,
   current_picture: Picture,
   uv: Label,
}

pub struct DailyWeatherBuilder {
    days: gtk::Box,
}


