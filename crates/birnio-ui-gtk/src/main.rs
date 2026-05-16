#![allow(dead_code)]

mod app;
mod features;
mod services;
mod shell;

use adw::prelude::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let application = adw::Application::builder()
        .application_id("dev.birnio.Birnio")
        .build();

    application.connect_activate(app::activate);
    application.run();

    Ok(())
}
