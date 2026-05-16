use gtk::prelude::*;

pub fn build() -> gtk::Widget {
    let editor = gtk::Box::new(gtk::Orientation::Vertical, 12);
    editor.set_margin_top(12);
    editor.set_margin_bottom(12);
    editor.set_margin_start(12);
    editor.set_margin_end(12);

    let url = gtk::Entry::builder()
        .placeholder_text("https://api.example.com")
        .hexpand(true)
        .build();
    let send = gtk::Button::with_label("Send");
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.append(&url);
    row.append(&send);

    editor.append(&row);
    editor.append(&gtk::TextView::new());
    editor.upcast()
}
