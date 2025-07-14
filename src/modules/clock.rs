use crate::Config;
use gtk4::prelude::{BoxExt, WidgetExt};

pub fn module_clock(_config: &Config) -> gtk4::Box {
    let mod_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    mod_box.set_css_classes(&["bar-module"]);
    let label = gtk4::Label::new(Some("Clock Module"));
    mod_box.append(&label);
    mod_box
}
