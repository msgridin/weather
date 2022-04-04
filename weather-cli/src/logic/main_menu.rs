use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::{read_input, State};
use crate::logic::weather_report::WeatherReport;
use crate::logic::select_date::SelectDate;
use crate::logic::select_location::SelectLocation;
use crate::logic::select_provider::SelectProvider;
use crate::logic::exit::Exit;

pub struct MainMenu;

impl State for MainMenu {
    fn update(&mut self, settings: &mut UserSettings, _: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        if settings.provider.is_none() {
            return Ok(Box::new(SelectProvider));
        } else if settings.location.is_none() {
            return Ok(Box::new(SelectLocation));
        }

        println!("\nPress key:
\t1) Weather report
\t2) Select a provider: <{}>
\t3) Select a city: <{}>
\t4) Select a date: <{}>
Other) Exit",
                 match settings.provider {
                     Some(ref provider) => provider.as_str(),
                     None => "-"
                 },
                 match settings.location.as_ref() {
                     Some(location) => location.to_string(),
                     None => "-".to_string(),
                 },
                 match settings.date {
                     Some(date) => date.format("%d-%m-%Y").to_string(),
                     None => "current".to_string(),
                 },
        );

        let selected = read_input()?;
        match selected.as_str() {
            "1" => Ok(Box::new(WeatherReport)),
            "2" => Ok(Box::new(SelectProvider)),
            "3" => Ok(Box::new(SelectLocation)),
            "4" => Ok(Box::new(SelectDate)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

