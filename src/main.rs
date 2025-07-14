use gdk4::Display;
use gtk4::{Application, ApplicationWindow, Orientation, Popover};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::info;

mod modules;

const APP_ID: &str = "org.hobbit125.hobbit-bar";

#[derive(Serialize, Deserialize, Debug, Default)]
struct ConfigModules {
    #[serde(default)]
    pub left: Vec<String>,
    #[serde(default)]
    pub center: Vec<String>,
    #[serde(default)]
    pub right: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    #[serde(default)]
    pub modules: ConfigModules,
}

fn main() -> glib::ExitCode {
    tracing_config::init!();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn get_module_map() -> HashMap<String, fn(&Config) -> gtk4::Box> {
    let mut map = HashMap::new();
    map.insert(
        String::from("active_window"),
        modules::module_active_window as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("battery"),
        modules::module_battery as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("bluetooth"),
        modules::module_bluetooth as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("clock"),
        modules::module_clock as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("hypr_workspaces"),
        modules::module_hypr_workspaces as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("media"),
        modules::module_media as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("volume"),
        modules::module_volume as fn(&Config) -> gtk4::Box,
    );
    map.insert(
        String::from("wifi"),
        modules::module_wifi as fn(&Config) -> gtk4::Box,
    );
    map
}

fn build_ui(app: &Application) {
    let provider = CssProvider::new();
    let config_text = std::fs::read_to_string("config.toml").unwrap();
    let config = toml::from_str::<Config>(&config_text).unwrap();
    let mod_map = get_module_map();

    info!("LEFT:   {:?}", config.modules.left);
    info!("CENTER: {:?}", config.modules.center);
    info!("RIGHT:  {:?}", config.modules.right);

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

    let left_box = gtk4::Box::new(Orientation::Horizontal, 10);
    let left_spacer = gtk4::Box::new(Orientation::Horizontal, 0);
    left_spacer.set_hexpand(true);
    let center_box = gtk4::Box::new(Orientation::Horizontal, 10);
    let right_spacer = gtk4::Box::new(Orientation::Horizontal, 0);
    right_spacer.set_hexpand(true);
    let right_box = gtk4::Box::new(Orientation::Horizontal, 10);

    let left_mods = config.modules.left.clone();
    let center_mods = config.modules.center.clone();
    let right_mods = config.modules.right.clone();

    for mod_name in left_mods {
        let f = mod_map.get(&mod_name).unwrap();
        let mod_box = f(&config);
        left_box.append(&mod_box);
    }

    for mod_name in center_mods {
        let f = mod_map.get(&mod_name).unwrap();
        let mod_box = f(&config);
        center_box.append(&mod_box);
    }

    for mod_name in right_mods {
        let f = mod_map.get(&mod_name).unwrap();
        let mod_box = f(&config);
        right_box.append(&mod_box);
    }

    window_box.append(&left_box);
    window_box.append(&left_spacer);
    window_box.append(&center_box);
    window_box.append(&right_spacer);
    window_box.append(&right_box);

    window.set_child(Some(&window_box));

    window.show();
}
