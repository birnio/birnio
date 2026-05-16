use gtk::prelude::*;

pub fn build() -> gtk::Widget {
    let group = gtk::Box::new(gtk::Orientation::Vertical, 6);
    group.append(&gtk::Label::new(Some("Environments")));
    group.append(&gtk::DropDown::from_strings(&["No environment"]));
    group.upcast()
}
