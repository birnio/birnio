use adw::prelude::*;

use crate::{features, shell};

pub fn build(application: &adw::Application) -> adw::ApplicationWindow {
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    root.append(&shell::header::build());

    let body = gtk::Paned::new(gtk::Orientation::Horizontal);
    body.set_start_child(Some(&shell::sidebar::build()));
    body.set_end_child(Some(&main_content()));
    body.set_resize_start_child(false);
    body.set_shrink_start_child(false);
    root.append(&body);

    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("Birnio")
        .default_width(1200)
        .default_height(760)
        .build();
    window.set_content(Some(&root));
    window
}

fn main_content() -> gtk::Widget {
    let split = gtk::Paned::new(gtk::Orientation::Horizontal);
    split.set_start_child(Some(&features::request_editor::component::build()));
    split.set_end_child(Some(&features::response_viewer::component::build()));
    split.set_resize_start_child(true);
    split.upcast()
}
