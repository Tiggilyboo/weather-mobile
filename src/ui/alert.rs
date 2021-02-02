
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
    pub container: gtk::Box,
    pub alerts: Vec<gtk::InfoBar>,
}

fn create_infobar_alert(alert: &WeatherAlert) -> InfoBar {
    let i = InfoBar::new();
    i.set_message_type(MessageType::Warning);
    i.set_show_close_button(true);
    i.connect_response(|info, response| {
        match response {
            gtk::ResponseType::Close => info.set_revealed(false),
            _ => {},
        }
    });

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
        let container = gtk::Box::new(gtk::Orientation::Vertical, 10);

        let mut wa = WeatherAlerts {
            container,
            alerts,
        };
        if let Some(data) = data {
            wa.populate(data);
        }

        wa
    }

    pub fn populate(&mut self, data: Vec<WeatherAlert>) {
        for alert in self.alerts.iter() {
            self.container.remove(alert);
        }
        self.alerts = data.iter()
            .map(|a| create_infobar_alert(a))
            .collect::<Vec<_>>();

        for alert in self.alerts.iter() {
            self.container.append(alert);
        }
    }
}
