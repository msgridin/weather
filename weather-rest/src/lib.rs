use tokio::runtime::Runtime;
use weather_common::models::location::Location;
use weather_common::models::{Provider, Source};
use weather_common::models::settings::UserSettings;
use weather_common::models::weather_report_data::WeatherReportData;

pub struct RestClient;

impl Source for RestClient {
    fn get_geo(&self, city: &str, settings: &UserSettings) -> Result<Location, anyhow::Error> {
        let runtime = Runtime::new().unwrap();
        let provider = get_provider(settings)?;
        let client = reqwest::Client::new();
        let url = provider.search_geo_url(city);
        let response = runtime.block_on(client.get(url).send())?;
        let status = response.status().as_u16();
        let body = runtime.block_on(response.text())?;
        if status != 200 {
            return Err(anyhow::anyhow!("Request failed with status code {}\n{}", status, body));
        }
        let location = provider.location_from_response(body, city)?;

        Ok(location)
    }


    fn get_weather(&self, settings: &UserSettings) -> Result<WeatherReportData, anyhow::Error> {
        let runtime = Runtime::new().unwrap();
        let provider = get_provider(settings)?;
        let client = reqwest::Client::new();
        let url = provider.get_weather_url(settings)?;
        let response = runtime.block_on(client.get(url).send())?;
        let status = response.status().as_u16();
        let body = runtime.block_on(response.text())?;
        if status != 200 {
            return Err(anyhow::anyhow!("Request failed with status code {}\n{}", status, body));
        }
        let weather_report = provider.report_from_response(body)?;

        Ok(weather_report)
    }
}

fn get_provider(settings: &UserSettings) -> Result<&Box<dyn Provider>, anyhow::Error> {
    let provider_name = match settings.provider.as_ref() {
        None => return Err(anyhow::anyhow!("No provider specified")),
        Some(provider) => provider,
    };

    let provider: &Box<dyn Provider> = settings.providers.get(provider_name).unwrap().clone();

    Ok(provider)
}
