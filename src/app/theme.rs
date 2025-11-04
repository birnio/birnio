#[derive(Debug, Clone)]
pub enum AppTheme {
    Dark,
    Light,
}

impl AppTheme {
    pub fn convert(&self) -> iced::Theme {
        match self {
            AppTheme::Dark => iced::Theme::Dracula,
            AppTheme::Light => iced::Theme::CatppuccinMocha,
        }
    }
}
