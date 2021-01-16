use serde::Deserialize;

#[derive(Deserialize)]
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

impl Units {
    pub fn unit_of_measure(&self) -> &str {
        match *self {
            Units::Imperial => "°F",
            Units::Metric => "°C",
        }
    }
}
