use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::State;
use crate::logic::main_menu::MainMenu;

pub struct WeatherReport;

impl State for WeatherReport {
    fn update(&mut self, settings: &mut UserSettings, source: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        println!("\nWeather report for {}, provided by {}. ({})",
                 match settings.location {
                     Some(ref location) => location.to_string(),
                     None => "-".to_string(),
                 },
                 match settings.provider {
                     Some(ref provider) => provider.as_str(),
                     None => "-"
                 },
                 match settings.date {
                     Some(date) => date.format("%d.%m.%Y").to_string(),
                     None => "today".to_string()
                 },
        );
        let report_data = source.get_weather(settings)?;

        println!("{}", report_data);
        Ok(Box::new(MainMenu))
    }
}


