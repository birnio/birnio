use gtk::prelude::*;

pub fn build() -> gtk::Widget {
    let group = gtk::Box::new(gtk::Orientation::Vertical, 6);
    group.append(&gtk::Label::new(Some("Collections")));

    let list = gtk::ListBox::new();
    list.append(&gtk::Label::new(Some("Example request")));
    group.append(&list);

    group.upcast()
}
