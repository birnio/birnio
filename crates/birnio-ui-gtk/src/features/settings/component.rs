use gtk::prelude::*;

pub fn build() -> gtk::Widget {
    let button = gtk::Button::with_label("Settings");
    button.upcast()
}
