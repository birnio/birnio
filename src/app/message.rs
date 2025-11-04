use crate::views::{home, settings};

#[derive(Clone, Debug)]
pub enum AppMessage {
    Home(home::Message),
    Settings(settings::Message),
}
