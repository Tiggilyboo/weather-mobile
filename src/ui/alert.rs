use crate::api::weather::{
    WeatherAlert,
};
use gtk::prelude::*;
use gtk::{
    Label,
    InfoBar,
    MessageType,
};

pub struct WeatherAlerts {
    pub container: gtk::ScrolledWindow,
    pub contents: gtk::Box,
    pub alerts: Vec<gtk::InfoBar>,
}

fn create_empty_alert() -> InfoBar {
    let i = InfoBar::new();
    i.set_message_type(MessageType::Other);

    let label = Label::new(Some("No alerts"));
    label.set_wrap(true);
    label.set_max_width_chars(80);

    i.add_child(&label);
    i
}

fn create_infobar_alert(alert: &WeatherAlert) -> InfoBar {
    let i = InfoBar::new();
    i.set_message_type(MessageType::Warning);

    let when = alert.when();
    let label = Label::new(Some(&format!("<b>{}</b> 
<small>({})</small>

{}

- <i>{}</i>", 
    alert.event, 
    when,
    alert.description,
    alert.sender_name)));

    label.set_use_markup(true);
    label.set_max_width_chars(80);
    label.set_wrap_mode(gtk::pango::WrapMode::Word);
    label.set_wrap(true);

    i.add_child(&label);
    i
}

impl WeatherAlerts {
    pub fn new(data: Option<Vec<WeatherAlert>>) -> Self {
        let alerts: Vec<InfoBar> = Vec::new();
        let contents = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let container = gtk::ScrolledWindow::new();
        container.set_child(Some(&contents));
        container.set_propagate_natural_width(true);
        container.set_propagate_natural_height(true);
        container.set_kinetic_scrolling(true);

        let mut wa = WeatherAlerts {
            container,
            contents,
            alerts,
        };
        if let Some(data) = data {
            wa.populate(data);
        }

        wa
    }

    pub fn populate(&mut self, data: Vec<WeatherAlert>) {
        for alert in self.alerts.iter() {
            self.contents.remove(alert);
        }
        self.alerts.clear();
        for alert_data in data.iter() {
            self.alerts.push(create_infobar_alert(alert_data));
        }
        if self.alerts.is_empty() {
            self.alerts.push(create_empty_alert());
        }
        for alert in self.alerts.iter() {
            self.contents.append(alert);
        }
    }
}
