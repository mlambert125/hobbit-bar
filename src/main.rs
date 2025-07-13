use gdk4::Display;
use gdk4_wayland::{WaylandDisplay, WaylandSurface};
use glib::clone;
use gtk4::{Application, ApplicationWindow, Orientation, Popover};
use gtk4::{CssProvider, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::path::Path;

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

    let button = gtk4::Button::new();
    let label = gtk4::Label::new(Some(""));
    let time_of_day = chrono::Local::now().format("%I:%M:%S %p").to_string();

    label.set_markup(&format!("<span font_desc=\"12.0\">{time_of_day}</span>"));
    label.add_css_class("time");

    // Create the popover (initially hidden)
    let popover_content = gtk4::Box::new(Orientation::Vertical, 4);
    popover_content.append(&gtk4::Label::new(Some("Today is...")));
    popover_content.append(&gtk4::Label::new(Some("Wednesday, July 10")));

    let popover = Popover::builder()
        .child(&popover_content)
        .has_arrow(true)
        .build();

    popover.add_css_class("popover");
    popover.set_autohide(true);
    popover.set_has_arrow(false);
    popover.set_parent(&label); // Anchor to the button

    let popover_button = gtk4::Button::with_label("Close");
    popover_button.add_css_class("popover-button");
    popover_content.append(&popover_button);
    popover_button.connect_clicked(clone!(
        #[weak]
        popover,
        move |_| {
            popover.popdown();
        }
    ));

    // Toggle on click
    button.connect_clicked(clone!(
        #[weak]
        popover,
        move |_| {
            if popover.is_visible() {
                popover.popdown();
            } else {
                popover.popup();
            }
        }
    ));

    button.set_child(Some(&label));
    button.set_halign(gtk4::Align::End);
    glib::timeout_add_seconds_local(1, move || -> glib::ControlFlow {
        let time_of_day = chrono::Local::now().format("%I:%M:%S %p").to_string();
        label.set_markup(&format!("<span font_desc=\"12.0\">{time_of_day}</span>"));
        glib::ControlFlow::Continue
    });

    window.connect_realize(|win| {
        let gdk_surface = win.surface().unwrap();
        let gdk_display = SurfaceExt::display(&gdk_surface);

        if let Some(wl_display) = gdk_display.downcast_ref::<WaylandDisplay>() {
            println!("Running on Wayland.");
            if let Some(wl_surface) = gdk_surface.downcast_ref::<WaylandSurface>() {
                println!("Wayland surface found.");
            }
        }
    });

    let window_box = gtk4::Box::new(Orientation::Horizontal, 10);
    window_box.set_halign(gtk4::Align::Fill);

    window_box.append(&button);

    window.set_child(Some(&window_box));

    window.show();
}
