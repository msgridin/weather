use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use weather_common::models::location::Location;
use weather_common::models::providers::Providers;
use weather_common::models::settings::UserSettings;
use weather_common::models::Storage;

pub struct FileSystemStorage;

impl Storage for FileSystemStorage {
    fn restore_user_settings(&self, path: &str, providers: Providers) ->  Result<UserSettings, anyhow::Error> {
        let json = fs::read_to_string(path).unwrap_or_default();
        let settings = UserSettingsFile::from(json);
        let settings = settings.to_user_settings(providers);
        Ok(settings)
    }

    fn save_user_settings(&self, path: &str, settings: &UserSettings) -> Result<(), anyhow::Error> {
        let settings = UserSettingsFile::from(settings);
        let json = serde_json::to_string_pretty(&settings)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsFile {
    pub date: Option<NaiveDate>,
    pub location: Option<Location>,
    pub provider: Option<String>,
    pub api_keys: HashMap<String, String>,
}

impl Default for UserSettingsFile {
    fn default() -> Self {
        UserSettingsFile {
            date: None,
            location: None,
            provider: None,
            api_keys: HashMap::new(),
        }
    }
}

impl From<String> for UserSettingsFile {
    fn from(s: String) -> Self {
        serde_json::from_str(&s).unwrap_or_default()
    }
}

impl From<&UserSettings> for UserSettingsFile {
    fn from(s: &UserSettings) -> Self {
        let mut api_keys = HashMap::new();
        for (name, provider) in s.providers.iter() {
            api_keys.insert(name.clone(), provider.data().api_key.clone());
        }

        UserSettingsFile {
            date: s.date,
            location: s.location.clone(),
            provider: s.provider.clone(),
            api_keys,
        }
    }
}

impl UserSettingsFile {
    pub fn to_user_settings(self, mut providers: Providers) -> UserSettings {

        for (name, provider) in providers.iter_mut() {
            let api_key = self.api_keys.get(name).unwrap_or(&"".to_string()).clone();
            provider.set_api_key(api_key);
        }

        UserSettings {
            date: self.date,
            location: self.location,
            provider: self.provider,
            providers,
        }
    }
}
