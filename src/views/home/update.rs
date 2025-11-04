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
        Message::ButtonClicked => (Task::none(), None),
    }
}
