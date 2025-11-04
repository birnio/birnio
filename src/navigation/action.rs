use super::Route;

/// Actions that can be performed on the navigation stack
#[derive(Clone, Debug)]
pub enum NavigationAction {
    Push(Route),
    Pop,
    // Replace(Route),
    // Reset(Route),
}
