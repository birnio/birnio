use iced::{Element, Task, Theme};

use super::AppMessage;
use crate::app::AppTheme;
use crate::navigation::{NavigationStack, Route};
use crate::views::{home, settings};

pub struct App {
    navigation: NavigationStack,
    theme: AppTheme,
}

impl App {
    pub fn new() -> (Self, Task<AppMessage>) {
        (
            App {
                navigation: NavigationStack::new(Route::home()),
                theme: AppTheme::Dark,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        crate::update! { self, message,
            AppMessage::Home(msg), Route::Home(state) => home::update(state, msg),
            AppMessage::Settings(msg), Route::Settings(state) => settings::update(state, msg),
        }
    }

    pub fn view(&self) -> Element<'_, AppMessage> {
        crate::route! { self,
            Route::Home(state), home::view(state) => AppMessage::Home,
            Route::Settings(state), settings::view(state) => AppMessage::Settings,
            Route::Workspaces, iced::widget::text("Workspaces - Coming soon") => AppMessage::Home,
            Route::Environments, iced::widget::text("Environments - Coming soon") => AppMessage::Home,
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone().convert()
    }
}
