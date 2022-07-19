use anyhow::Result;
use std::{fs::create_dir_all, path::PathBuf};
use tauri::App;

use crate::database::{db, execute};

const IDENTIFIER: &str = "com.abtswath.rubick";

pub fn app_dir() -> PathBuf {
    tauri::api::path::app_dir(&tauri::Config::default()).unwrap().join(IDENTIFIER)
}

pub fn image_dir() -> PathBuf {
    app_dir().join("images")
}

pub fn setup(app: &mut App) -> Result<()> {
    let app_dir = match app.path_resolver().app_dir() {
        None => PathBuf::new(),
        Some(p) => p.join(IDENTIFIER),
    };
    if !app_dir.exists() {
        create_dir_all(&app_dir)?;
    }
    let image_dir = image_dir();
    if !image_dir.exists() {
        create_dir_all(image_dir)?;
    }
    Ok(())
}

pub fn initialized() -> bool {
    if !db().exists() {
        return false;
    }

    let result = execute(|connection| {
        let res = connection.query_row("select count(*) as c from resources", [], |row| {
            let size = row.get::<&str, i32>("c")?;
            Ok(size)
        })?;
        Ok(res)
    });

    match result {
        Err(_) => false,
        Ok(size) => size > 0,
    }
}
