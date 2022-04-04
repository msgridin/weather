use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::{read_input, State};
use crate::logic::main_menu::MainMenu;

pub struct SelectLocation;

impl State for SelectLocation {
    fn update(&mut self, settings: &mut UserSettings, source: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        println!("\nEnter the city you want to see the weather");

        let selected = read_input()?;

        let location = source.get_geo(selected.as_str(), settings);

        let location = match location {
            Ok(location) => location,
            Err(e) => {
                println!("\n{}", e);
                println!("Please try again.");
                return Ok(Box::new(SelectLocation));
            }
        };

        settings.location = Some(location);

        Ok(Box::new(MainMenu))
    }
}


