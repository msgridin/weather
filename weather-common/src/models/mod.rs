use std::fmt::Display;
use crate::models::location::Location;
use crate::models::weather_report_data::WeatherReportData;
use serde::{Serialize, Deserialize};
use crate::models::providers::Providers;
use crate::models::settings::UserSettings;

pub mod location;
pub mod weather_report_data;
pub mod settings;
pub mod providers;

pub trait Storage {
    fn restore_user_settings(&self, path: &str, providers: Providers) -> Result<UserSettings, anyhow::Error>;
    fn save_user_settings(&self, path: &str, user_settings: &UserSettings) -> Result<(), anyhow::Error>;
}

pub trait Source {
    fn get_geo(&self, city: &str, settings: &UserSettings) -> Result<Location, anyhow::Error>;
    fn get_weather(&self, settings: &UserSettings) -> Result<WeatherReportData, anyhow::Error>;
}

pub trait Provider {
    fn data(&self) -> &ProviderData;
    fn set_api_key(&mut self, api_key: String);
    fn location_from_response(&self, body: String, city: &str) -> Result<Location, anyhow::Error>;
    fn report_from_response(&self, body: String) -> Result<WeatherReportData, anyhow::Error>;

    fn search_geo_url(&self, city: &str) -> String {
        let data = self.data();
        let mut url = data.search_geo_url.clone();
        url = url.replace("<API_KEY>", data.api_key.as_str());
        url = url.replace("<CITY>", city);
        url
    }

    fn get_weather_url(&self, settings: &UserSettings) -> Result<String, anyhow::Error> {
        let location = match settings.location.as_ref() {
            None => return Err(anyhow::anyhow!("No city specified")),
            Some(location) => location,
        };

        let data = self.data();
        let mut url = match settings.date {
            Some(date) => data.history_weather_url.replace("<DATE>", date.format("%Y-%m-%d").to_string().as_str()),
            None => data.current_weather_url.clone(),
        };
        url = url.replace("<API_KEY>", data.api_key.as_str());
        url = url.replace("<LATITUDE>", location.lat.to_string().as_str());
        url = url.replace("<LONGITUDE>", location.lon.to_string().as_str());
        Ok(url)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderData {
    pub name: String,
    pub search_geo_url: String,
    pub current_weather_url: String,
    pub history_weather_url: String,
    pub api_key: String,
}

impl Display for ProviderData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WeatherProvider {}", self.name)
    }
}

impl PartialEq for ProviderData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

