/// Internal macro - handles message dispatching to views
#[doc(hidden)]
#[macro_export]
macro_rules! __dispatch_to_view {
    ($nav:expr, $route_pattern:pat, $update_call:expr) => {{
        if let $route_pattern = $nav.current_mut() {
            let (task, nav_action) = $update_call;
            if let Some(action) = nav_action {
                $nav.apply_action(action);
            }
            return task;
        }
    }};
}

/// Routes messages to their corresponding view update functions
///
/// Usage inside `impl App`:
/// ```
/// pub fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
///     update! { self, message,
///         AppMessage::Home(msg), Route::Home(state) => home::update(state, msg),
///         AppMessage::Settings(msg), Route::Settings(state) => settings::update(state, msg),
///     }
/// }
/// ```
///
/// Automatically handles navigation actions returned by view updates
#[macro_export]
macro_rules! update {
    ($self:ident, $message:ident, $($msg_pattern:pat, $route_pattern:pat => $update_call:expr),+ $(,)?) => {{
        match $message {
            $(
                $msg_pattern => {
                    $crate::__dispatch_to_view!($self.navigation, $route_pattern, $update_call);
                }
            )+
        }
        iced::Task::none()
    }};
}

/// Renders the current route as a view
///
/// Usage inside `impl App`:
/// ```
/// pub fn view(&self) -> Element<'_, AppMessage> {
///     route! { self,
///         Route::Home(state), home::view(state) => AppMessage::Home,
///         Route::Settings(state), settings::view(state) => AppMessage::Settings,
///     }
/// }
/// ```
///
/// Automatically maps each view's messages to the app-level message type
#[macro_export]
macro_rules! route {
    ($self:ident, $($route_pattern:pat, $view_call:expr => $msg_wrapper:path),+ $(,)?) => {{
        match $self.navigation.current() {
            $(
                $route_pattern => {
                    use iced::Element;
                    let element: Element<'_, _> = $view_call.into();
                    element.map($msg_wrapper)
                }
            )+
        }
    }};
}
