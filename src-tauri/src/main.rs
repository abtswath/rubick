#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{api::dialog, Manager, WindowBuilder, WindowUrl};

use crate::api::{search, resource};

mod application;
mod database;
mod initialize;
mod response;
mod yyets;
mod api;
mod douban;

#[async_std::main]
async fn main() {
    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .setup(move |app| {
            application::setup(app)?;
            if !application::initialized() {
                let _ = WindowBuilder::new(app, "initialize", WindowUrl::App("initialize".into()))
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
            } else {
                let _ = WindowBuilder::new(app, "main", WindowUrl::default())
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
            }
            Ok(())
        })
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            search,
            resource
        ])
        .build(context)
        .expect("发生未知错误！");
    app.run(|app_handle, e| match e {
        tauri::RunEvent::Exit => {
            if let Some(_window) = app_handle.get_window("initialize") {
                initialize::clear();
            }
        },
        tauri::RunEvent::Ready => {
            if let Some(window) = app_handle.get_window("initialize") {
                match database::setup()
                    .and_then(|_| {
                        window.show()?;
                        Ok(())
                    })
                    .and_then(|_| {
                        initialize::initialize(window.clone());
                        Ok(())
                    }) {
                    Ok(_) => {}
                    Err(e) => dialog::message(Some(&window), "Error", e.to_string().as_str()),
                };
                return;
            }
            if let Some(window) = app_handle.get_window("main") {
                match window.show() {
                    Ok(_) => {}
                    Err(e) => dialog::message(Some(&window), "Error", e.to_string().as_str()),
                };
                return;
            };
            panic!("There is no any window active");
        }
        _ => {}
    });
}
