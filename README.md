# hobbit-bar

Just some old code for reference for now:

```rust
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
```
