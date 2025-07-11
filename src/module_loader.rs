use std::{cell::RefCell, fs};

use mlua::{Lua, Table};
use roxmltree::Document;
use tracing::debug;

pub fn load_module(name: &str) -> (gtk4::Box, Lua) {
    let lua = Lua::new();
    let builder = gtk4::Builder::new();
    let scope = gtk4::BuilderRustScope::new();

    let code =
        fs::read_to_string(format!("./bar-modules/{name}.lua")).expect("Failed to read Lua file");

    let module: Table = lua.load(&code).eval().expect("Lua code failed to evaluate");
    let module_rc = RefCell::new(module);

    let xml =
        fs::read_to_string(format!("./bar-modules/{name}.ui")).expect("Failed to read UI file");
    let doc = Document::parse(&xml).expect("Failed to parse XML");

    for node in doc.descendants().filter(|n| n.has_tag_name("signal")) {
        let module_rc = module_rc.clone();
        if let Some(handler) = node.attribute("handler") {
            let handler = handler.to_owned();

            scope.add_callback(handler.clone(), move |_| {
                debug!("HANDLER: {handler}");
                let lua_fn = module_rc
                    .borrow()
                    .get::<mlua::Function>(handler.clone())
                    .expect("Handler not found");

                lua_fn.call::<()>(()).expect("Lua error");

                None
            });
        }
    }
    builder.set_scope(Some(&scope));
    builder
        .add_from_file(format!("./bar-modules/{name}.ui"))
        .unwrap();

    let module = builder.object::<gtk4::Box>("module").unwrap();

    (module, lua)
}
