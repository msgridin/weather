use serde::{Deserialize, Serialize};
use crate::models::location::Location;
use crate::models::{Provider, ProviderData};
use crate::models::weather_report_data::WeatherReportData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherApiProvider {
    pub data: ProviderData,
}

impl Provider for WeatherApiProvider {
    fn data(&self) -> &ProviderData {
        &self.data
    }

    fn set_api_key(&mut self, api_key: String) {
        self.data.api_key = api_key;
    }

    fn location_from_response(&self, body: String, city: &str) -> Result<Location, anyhow::Error> {
        let response: Vec<GeoResponse> = serde_json::from_str(&body)?;

        for result in response {
            if result.name.to_lowercase() == city.to_lowercase() {
                return Ok(Location {
                    city: result.name,
                    country: result.country,
                    lat: result.lat,
                    lon: result.lon,
                });
            }
        }

        Err(anyhow::anyhow!("No city found"))
    }

    fn report_from_response(&self, body: String) -> Result<WeatherReportData, anyhow::Error> {
        let response: WeatherResponse = serde_json::from_str(&body)?;

        let weather_report = WeatherReportData {
            temp: response.current.temp_c,
            pressure: response.current.pressure_mb.round() as u32,
            humidity: response.current.humidity,
        };

        Ok(weather_report)
    }
}

impl Default for WeatherApiProvider {
    fn default() -> Self {
        WeatherApiProvider {
            data: ProviderData {
                name: "Weather API".to_string(),
                search_geo_url: "https://api.weatherapi.com/v1/search.json?key=<API_KEY>&q=<CITY>".to_string(),
                current_weather_url: "https://api.weatherapi.com/v1/current.json?key=<API_KEY>&q=<LATITUDE>,<LONGITUDE>".to_string(),
                history_weather_url: "https://api.weatherapi.com/v1/history.json?key=<API_KEY>&q=<LATITUDE>,<LONGITUDE>&dt=<DATE>".to_string(),
                api_key: "".to_string(),
            }
        }
    }
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct GeoResponse {
    pub id: i32,
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub url: String
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct WeatherResponse {
    pub current: WeatherCurrent,
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct WeatherCurrent {
    pub wind_kph: f32,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub humidity: u32,
    pub cloud: u32,
    pub feelslike_c: f32,
    pub temp_c: f32,
}

