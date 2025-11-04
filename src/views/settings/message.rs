use crate::navigation::NavigationAction;

#[derive(Clone, Debug)]
pub enum Message {
    Navigate(NavigationAction),
    ToggleSetting(String),
}
