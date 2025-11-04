use crate::views::{home, settings};
use birnio_macros::RouteConstructors;

#[derive(Clone, Debug, RouteConstructors)]
pub enum Route {
    Home(home::State),
    Settings(settings::State),
    Workspaces,
    Environments,
}
