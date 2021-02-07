use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub enum Units {
    Metric,
    Imperial
}

impl std::fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Units::Imperial => write!(f, "imperial"),
            Units::Metric => write!(f, "metric"),
        }
    }
}

impl PartialEq for Units {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Units {
    pub fn temperature_unit(&self) -> &str {
        match *self {
            Units::Imperial => "°F",
            Units::Metric => "°C",
        }
    }
    pub fn speed_unit(&self) -> &str {
        match *self {
            Units::Imperial => "mph",
            Units::Metric => "m/s",
        }
    }
    pub fn volume_unit(&self) -> &str {
        match *self {
            Units::Imperial => "mm",
            Units::Metric => "mm",
        }
    }
    pub fn temperature_value<T: std::fmt::Display>(&self, value: T) -> String {
        format!("{} {}", value, self.temperature_unit())
    }
    pub fn speed_value<T: std::fmt::Display>(&self, value: T) -> String {
        format!("{} {}", value, self.speed_unit())
    }
    pub fn volume_value<T: std::fmt::Display>(&self, value: T) -> String {
        format!("{} {}", value, self.volume_unit())
    }
}
