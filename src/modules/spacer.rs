use crate::Config;
use gtk4::prelude::{BoxExt, WidgetExt};

pub fn module_spacer(_config: &Config) -> gtk4::Box {
    let mod_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    mod_box.set_hexpand(true);
    mod_box
}
