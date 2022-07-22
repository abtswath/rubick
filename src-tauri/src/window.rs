use anyhow::Result;
use tauri::{Manager, Runtime, Window, WindowBuilder, WindowUrl};

pub fn create_main_window<M: Manager<R>, R: Runtime>(manager: &M) -> Result<Window<R>> {
    let window = WindowBuilder::new(manager, "main", WindowUrl::default())
        .title("Rubick")
        .resizable(true)
        .decorations(false)
        .fullscreen(false)
        .always_on_top(false)
        .inner_size(800.0, 600.0)
        .min_inner_size(800.0, 600.0)
        .visible(false)
        .center()
        .build()?;
    Ok(window)
}

pub fn create_initialize_window<M: Manager<R>, R: Runtime>(manager: &M) -> Result<Window<R>> {
    let window = WindowBuilder::new(manager, "initialize", WindowUrl::App("initialize".into()))
        .title("Rubick")
        .resizable(false)
        .decorations(false)
        .fullscreen(false)
        .maximized(false)
        .resizable(false)
        .always_on_top(false)
        .inner_size(520.0, 360.0)
        .skip_taskbar(false)
        .visible(false)
        .center()
        .build()?;
    Ok(window)
}
