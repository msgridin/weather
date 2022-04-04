use serde::{Deserialize, Serialize};
use crate::models::location::Location;
use crate::models::{Provider, ProviderData};
use crate::models::weather_report_data::WeatherReportData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWeatherProvider {
    pub data: ProviderData,
}

impl Provider for OpenWeatherProvider {
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
            temp: response.main.temp,
            pressure: response.main.pressure,
            humidity: response.main.humidity,
        };

        Ok(weather_report)
    }
}

impl Default for OpenWeatherProvider {
    fn default() -> Self {
        OpenWeatherProvider {
            data: ProviderData {
                name: "Open Weather".to_string(),
                search_geo_url: "https://api.openweathermap.org/geo/1.0/direct?q=<CITY>&limit=5&appid=<API_KEY>".to_string(),
                current_weather_url: "https://api.openweathermap.org/data/2.5/weather?units=metric&lat=<LATITUDE>&lon=<LONGITUDE>&appid=<API_KEY>".to_string(),
                history_weather_url: "https://history.openweathermap.org/data/2.5/history/city?units=metric&lat=<LATITUDE>&lon=<LONGITUDE>&type=hour&start=<DATE>&end=<DATE>&appid=<API_KEY>".to_string(),
                api_key: "".to_string(),
            }
        }
    }
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct GeoResponse {
    pub name: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct WeatherResponse {
    pub main: WeatherMain,
}

#[derive(serde_derive::Deserialize, Clone, Debug)]
pub struct WeatherMain {
    pub temp: f32,
    pub pressure: u32,
    pub humidity: u32,
}

