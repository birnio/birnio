use iced::Task;

use super::{Message, State};
use crate::app::AppMessage;
use crate::navigation::NavigationAction;

pub fn update(
    _state: &mut State,
    message: Message,
) -> (Task<AppMessage>, Option<NavigationAction>) {
    match message {
        Message::Navigate(action) => (Task::none(), Some(action)),
        Message::ToggleSetting(setting_name) => {
            println!("Toggling setting: {}", setting_name);
            (Task::none(), None)
        }
    }
}
