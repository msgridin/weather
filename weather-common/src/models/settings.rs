use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::location::Location;
use crate::models::providers::Providers;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub date: Option<NaiveDate>,
    pub location: Option<Location>,
    pub provider: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub providers: Providers,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            date: None,
            location: None,
            provider: None,
            providers: Default::default(),
        }
    }
}

