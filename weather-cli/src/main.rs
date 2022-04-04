use logic::State;
use logic::main_menu::MainMenu;
use weather_common::models::{Source, Storage};
use weather_common::models::providers::Providers;
use weather_rest::RestClient;
use crate::data::fs_storage::FileSystemStorage;

mod data;
mod logic;

// Filename for storage of user settings
const USER_SETTINGS_FILE: &str = "user_settings.json";

// The logic of the application is designed using a state machine
// User actions set the state of the state machine
// Each state has its own user action handler
fn main() {

    // Dependence injection
    // Source is responsible for fetching weather data
    // Can be any object that implements the Source trait
    let source: Box<dyn Source> = Box::new(RestClient);
    // Storage is responsible for save/restore user settings
    // Can be any object that implements the Storage trait
    let storage: Box<dyn Storage> = Box::new(FileSystemStorage);

    match run(&source, &storage) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn run(source: &Box<dyn Source>, storage: &Box<dyn Storage>) -> Result<(), anyhow::Error> {

    // Get available providers list
    let providers = Providers::default();

    // Read user settings from storage
    let mut settings = storage.restore_user_settings(USER_SETTINGS_FILE, providers)?;

    // MainMenu is start state of the state machine
    let mut state: Box<dyn State> = Box::new(MainMenu);
    while !state.exit() {

        // Render ui, get user input and update state
        state = state.update(&mut settings, source)?;
    }

    // Save user settings to storage
    storage.save_user_settings(USER_SETTINGS_FILE, &settings)?;
    Ok(())
}
