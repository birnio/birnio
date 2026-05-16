use gtk::prelude::*;

use crate::features;

pub fn build() -> gtk::Widget {
    let sidebar = gtk::Box::new(gtk::Orientation::Vertical, 12);
    sidebar.set_width_request(280);
    sidebar.set_margin_top(12);
    sidebar.set_margin_bottom(12);
    sidebar.set_margin_start(12);
    sidebar.set_margin_end(12);

    sidebar.append(&features::collections::component::build());
    sidebar.append(&features::environments::component::build());
    sidebar.append(&features::settings::component::build());

    sidebar.upcast()
}
