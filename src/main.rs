use gdk4::Display;
use gtk4::{Application, ApplicationWindow, Orientation};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::info;

mod modules;

const APP_ID: &str = "org.hobbit125.hobbit-bar";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum BarModule {
    ActiveWindow,
    Battery,
    Bluetooth,
    Clock,
    HyprWorkspaces,
    Media,
    Volume,
    Wifi,
    Spacer,
}

impl BarModule {
    pub fn call(&self, config: &Config) -> gtk4::Box {
        match self {
            BarModule::ActiveWindow => modules::module_active_window(config),
            BarModule::Battery => modules::module_battery(config),
            BarModule::Bluetooth => modules::module_bluetooth(config),
            BarModule::Clock => modules::module_clock(config),
            BarModule::HyprWorkspaces => modules::module_hypr_workspaces(config),
            BarModule::Media => modules::module_media(config),
            BarModule::Volume => modules::module_volume(config),
            BarModule::Wifi => modules::module_wifi(config),
            BarModule::Spacer => modules::module_spacer(config),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    #[serde(default)]
    pub modules: Vec<BarModule>,
}

fn main() -> glib::ExitCode {
    tracing_config::init!();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let provider = CssProvider::new();
    let config_text = std::fs::read_to_string("config.toml").unwrap();
    let config = toml::from_str::<Config>(&config_text).unwrap();

    info!("Modules:   {:?}", config.modules);

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

    let window_box = gtk4::Box::new(Orientation::Horizontal, 0);
    window_box.set_halign(gtk4::Align::Fill);

    for bar_mod in config.modules.clone() {
        let mod_box = bar_mod.call(&config);
        window_box.append(&mod_box);
    }

    window.set_child(Some(&window_box));

    window.show();
}
