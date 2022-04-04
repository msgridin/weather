use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use crate::models::{Provider, ProviderData};

pub mod open_weather_provider;
pub mod weather_api_provider;

pub struct Providers {
    pub providers: HashMap<String, Box<dyn Provider>>,
}

impl Debug for Providers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (name, provider) in self.providers.iter() {
            write!(f, "{}: {:?}\n", name, provider.data())?;
        }
        write!(f, "}}")
    }
}

impl Default for Providers {
    fn default() -> Self {
        let open_weather: Box<dyn Provider> = Box::new(open_weather_provider::OpenWeatherProvider::default());
        let weather_api: Box<dyn Provider> = Box::new(weather_api_provider::WeatherApiProvider::default());

        let providers = HashMap::from([
            (open_weather.data().name.clone(), open_weather),
            (weather_api.data().name.clone(), weather_api)]);

        Providers { providers }
    }
}

impl Providers {
    pub fn _get_provider(&self, name: &str) -> Option<&Box<dyn Provider>> {
        self.providers.get(name)
    }

    pub fn get_provider_mut(&mut self, name: &str) -> Option<&mut Box<dyn Provider>> {
        self.providers.get_mut(name)
    }

    pub fn get_provider_names(&self) -> Vec<String> {
        self.providers.keys().map(|k| k.clone()).collect()
    }

    pub fn _get_provider_data(&self, name: &str) -> Option<&ProviderData> {
        self.providers.get(name).map(|p| p.data())
    }
}

impl Deref for Providers {
    type Target =HashMap<String, Box<dyn Provider>>;

    fn deref(&self) -> &Self::Target {
        &self.providers
    }
}

impl DerefMut for Providers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.providers
    }
}

