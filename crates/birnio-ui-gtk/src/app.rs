use adw::prelude::*;

use crate::shell;

pub fn activate(application: &adw::Application) {
    let window = shell::window::build(application);
    window.present();
}
