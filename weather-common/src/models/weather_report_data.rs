use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherReportData {
    pub temp: f32,
    pub pressure: u32,
    pub humidity: u32,
}

impl Display for WeatherReportData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "temperature: {} C,\npressure: {} mm Hg,\nhumidity: {} %rh", self.temp, self.pressure, self.humidity)
    }
}
