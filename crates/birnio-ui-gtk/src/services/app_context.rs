#[derive(Clone, Debug)]
pub struct AppContext {
    pub app_name: &'static str,
}

impl Default for AppContext {
    fn default() -> Self {
        Self { app_name: "Birnio" }
    }
}
