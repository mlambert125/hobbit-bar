use std::{cell::RefCell, fs};

use mlua::{Lua, Table};
use roxmltree::Document;
use tracing::{debug, error, info, warn};

pub fn load_module(name: &str) -> anyhow::Result<(gtk4::Box, Lua)> {
    let lua = Lua::new();
    let builder = gtk4::Builder::new();
    let scope = gtk4::BuilderRustScope::new();

    let code = fs::read_to_string(format!("./bar-modules/{name}.lua"))?;
    attach_global_functions(&lua)?;

    let module: Table = lua
        .load(&code)
        .eval()
        .map_err(|err| anyhow::anyhow!("{err}"))?;
    let module_rc = RefCell::new(module);

    let xml = fs::read_to_string(format!("./bar-modules/{name}.ui"))?;
    let doc = Document::parse(&xml).expect("Failed to parse XML");

    for node in doc.descendants().filter(|n| n.has_tag_name("signal")) {
        let module_rc = module_rc.clone();
        if let Some(handler) = node.attribute("handler") {
            let handler = handler.to_owned();

            scope.add_callback(handler.clone(), move |_| {
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

    Ok((module, lua))
}

pub fn attach_global_functions(lua: &Lua) -> anyhow::Result<()> {
    lua.globals()
        .set(
            "debug",
            lua.create_function(|_, msg: String| {
                debug!(msg);
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!(format!("{err}")))?,
        )
        .map_err(|err| anyhow::anyhow!(format!("{err}")))?;
    lua.globals()
        .set(
            "info",
            lua.create_function(|_, msg: String| {
                info!(msg);
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!(format!("{err}")))?,
        )
        .map_err(|err| anyhow::anyhow!(format!("{err}")))?;
    lua.globals()
        .set(
            "warn",
            lua.create_function(|_, msg: String| {
                warn!(msg);
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!(format!("{err}")))?,
        )
        .map_err(|err| anyhow::anyhow!(format!("{err}")))?;
    lua.globals()
        .set(
            "error",
            lua.create_function(|_, msg: String| {
                error!(msg);
                Ok(())
            })
            .map_err(|err| anyhow::anyhow!(format!("{err}")))?,
        )
        .map_err(|err| anyhow::anyhow!(format!("{err}")))?;

    Ok(())
}
