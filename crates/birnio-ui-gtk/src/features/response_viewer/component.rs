use gtk::prelude::*;

pub fn build() -> gtk::Widget {
    let viewer = gtk::Box::new(gtk::Orientation::Vertical, 12);
    viewer.set_margin_top(12);
    viewer.set_margin_bottom(12);
    viewer.set_margin_start(12);
    viewer.set_margin_end(12);
    viewer.append(&gtk::Label::new(Some("Response")));
    viewer.append(&gtk::TextView::new());
    viewer.upcast()
}
