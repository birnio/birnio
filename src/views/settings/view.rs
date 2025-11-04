use iced::Element;
use iced::widget::{button, column, text};

use super::{Message, State};
use crate::navigation::NavigationAction;

pub fn view(_state: &State) -> Element<'_, Message> {
    column![
        text("Settings Screen").size(24),
        button("Go Back").on_press(Message::Navigate(NavigationAction::Pop)),
        button("Toggle Example Setting").on_press(Message::ToggleSetting("example".to_string())),
    ]
    .spacing(10)
    .padding(20)
    .into()
}
