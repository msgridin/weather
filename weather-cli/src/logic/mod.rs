use std::io;
use weather_common::models::settings::UserSettings;
use weather_common::models::Source;

pub mod main_menu;
mod weather_report;
mod exit;
mod select_date;
mod select_location;
mod select_provider;

pub trait State {
    fn update(&mut self, settings: &mut UserSettings, source: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error>;
    fn exit(&self) -> bool {
        false
    }
}

fn read_input() -> Result<String, anyhow::Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let selected = buf.trim();
    println!("Selected: {}", selected);
    Ok(selected.to_string())
}