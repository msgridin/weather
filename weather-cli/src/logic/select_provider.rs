use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::{read_input, State};
use crate::logic::main_menu::MainMenu;

pub struct SelectProvider;

impl State for SelectProvider {
    fn update(&mut self, settings: &mut UserSettings, _: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        println!("\nSelect a weather service provider or press 'c' to cancel:");

        let provider_names = settings.providers.get_provider_names();
        for (i, name) in provider_names.iter().enumerate() {
            println!("\t{}) {}", i + 1, name);
        }
        let selected = read_input()?;

        if selected == "c" {
            return Ok(Box::new(MainMenu));
        }

        let index = match selected.parse::<usize>() {
            Ok(selected) => selected - 1,
            Err(_) => {
                println!("Invalid selection");
                return Ok(Box::new(SelectProvider));
            }
        };

        if index >= provider_names.len() {
            println!("Invalid range");
            return Ok(Box::new(SelectProvider));
        };

        let provider_name = provider_names[index].clone();
        settings.provider = Some(provider_name.clone());

        println!("\nEnter new API key for {} or press 'c' to continue:", provider_name.as_str());
        let selected = read_input()?;

        if selected == "c" {
            return Ok(Box::new(MainMenu));
        }

        let provider = settings.providers.get_provider_mut(provider_name.as_str());

        let state: Box<dyn State> = match provider {
            Some(provider) => {
                provider.set_api_key(selected);
                Box::new(MainMenu)
            },
            None => {
                println!("Not found provider: {}", provider_name);
                Box::new(SelectProvider)
            }
        };

        Ok(state)
    }
}


