use iced::Element;
use iced::widget::{button, column, text};

use super::{Message, State};
use crate::navigation::{NavigationAction, Route};

pub fn view(_state: &State) -> Element<'_, Message> {
    column![
        text("Home Screen").size(24),
        button("Go to Settings")
            .on_press(Message::Navigate(NavigationAction::Push(Route::settings()))),
        button("Click Me").on_press(Message::ButtonClicked),
    ]
    .spacing(10)
    .padding(20)
    .into()
}
