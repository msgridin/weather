use chrono::NaiveDate;
use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::{read_input, State};
use crate::logic::main_menu::MainMenu;

pub struct SelectDate;

impl State for SelectDate {
    fn update(&mut self, settings: &mut UserSettings, _: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        println!("\nEnter the date (dd-mm-yyyy) or press 'c' to set current date:");

        let selected = read_input()?;

        if selected == "c" {
            settings.date = None;
            return Ok(Box::new(MainMenu));
        }

        let date = NaiveDate::parse_from_str(&selected, "%d-%m-%Y");

        let state: Box<dyn State> = match date {
            Ok(date) => {
                settings.date = Some(date);
                Box::new(MainMenu)
            },
            Err(_) => {
                println!("Invalid date format");
                Box::new(SelectDate)
            }
        };

        Ok(state)
    }
}


