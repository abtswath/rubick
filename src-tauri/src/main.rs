#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{api::dialog, Manager};

use crate::{
    api::{resource, search, favorite, un_favorite, favorites},
    window::{create_initialize_window, create_main_window},
};

mod api;
mod application;
mod database;
mod douban;
mod initialize;
mod response;
mod window;
mod yyets;

#[async_std::main]
async fn main() {
    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .setup(move |app| {
            if app.windows().len() > 0 {
                if let Some(window) = app.windows().values().next() {
                    window.set_focus()?;
                }
                return Ok(());
            }
            application::setup(app)?;
            database::setup()?;
            if !application::initialized() {
                let _ = create_initialize_window(app)?;
            } else {
                let _ = create_main_window(app)?;
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
            resource,
            favorites,
            favorite,
            un_favorite
        ])
        .build(context)
        .expect("发生未知错误！");
    app.run(|app_handle, e| match e {
        tauri::RunEvent::Exit => {
            if let Some(_window) = app_handle.get_window("initialize") {
                initialize::clear();
            }
        }
        tauri::RunEvent::Ready => {
            if let Some(window) = app_handle.get_window("initialize") {
                match window.show()
                    .and_then(|_| {
                        initialize::initialize(window.clone(), app_handle.app_handle());
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
