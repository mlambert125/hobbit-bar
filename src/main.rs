use std::path::Path;
use std::sync::{Mutex, RwLock};

use gdk4::Display;
use gtk4::{Application, ApplicationWindow, Orientation};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use mlua::{Lua, Table};
use tracing::debug;

use crate::module_loader::load_module;

mod module_loader;

const APP_ID: &str = "org.hobbit125.hobbit-bar";

fn main() -> glib::ExitCode {
    tracing_config::init!();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let provider = CssProvider::new();

    provider.load_from_path(Path::new("style.css"));

    let display = Display::default().expect("Could not connect to a display");

    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(1900)
        .default_height(30)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Bottom);
    window.auto_exclusive_zone_enable();
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.set_anchor(Edge::Top, true);

    let window_box = gtk4::Box::new(Orientation::Horizontal, 10);
    window_box.set_halign(gtk4::Align::Fill);
    window.set_child(Some(&window_box));

    // TODO: Load from toml config instead of autoloading everyting in the folder
    for entry in std::fs::read_dir("./bar-modules").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file()
            && let Some(extension) = path.extension()
            && extension == "ui"
            && let Some(file_stem) = path.file_stem()
            && let Some(module_name) = file_stem.to_str()
        {
            debug!("Loading module: {module_name}");
            let (module, lua) = load_module(module_name);
            window_box.append(&module);
            Box::leak(Box::new(lua));
        }
    }

    window.show();
}
