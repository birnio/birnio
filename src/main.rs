mod app;
mod components;
mod navigation;
mod services;
mod state;
mod styles;
mod views;

use app::App;

fn main() -> iced::Result {
    iced::application("Meu projetinho", App::update, App::view)
        .theme(App::theme)
        .run_with(App::new)
}
