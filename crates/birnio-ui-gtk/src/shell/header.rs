use adw::prelude::*;

pub fn build() -> adw::HeaderBar {
    let header = adw::HeaderBar::new();
    let title = gtk::Label::new(Some("Birnio"));
    title.add_css_class("title-3");
    header.set_title_widget(Some(&title));
    header
}
