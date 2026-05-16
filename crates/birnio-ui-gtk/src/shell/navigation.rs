#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Route {
    #[default]
    RequestEditor,
    Environments,
    Settings,
}
