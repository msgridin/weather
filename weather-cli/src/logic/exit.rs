use weather_common::models::settings::UserSettings;
use weather_common::models::Source;
use crate::logic::State;

pub struct Exit;

impl State for Exit {
    fn update(&mut self, _: &mut UserSettings, _: &Box<dyn Source>) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
