use anyhow::Result;
use rusqlite::{params, Transaction};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::remove_file, path::PathBuf, thread};
use tauri::{api::dialog, AppHandle, Window};

use crate::{
    database::{self, connect, execute},
    response::Response,
    window::create_main_window,
    yyets::{self, Item, RecordData, YYeTsSeason},
};

pub struct NameOnly {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResponseData<T> {
    step: String,
    data: T,
}

impl<T> ResponseData<T> {
    pub fn new(step: &str, data: T) -> Self {
        ResponseData {
            step: step.to_string(),
            data,
        }
    }
}

pub fn initialize(window: Window, app_handle: AppHandle) {
    thread::spawn(move || {
        tauri::async_runtime::block_on(async move {
            let result = yyets::download(window.clone())
                .await
                .and_then(|db| import_series(&window, db));

            match result {
                Ok(_) => {
                    match create_main_window(&app_handle) {
                        Ok(main_window) => {
                            let _ = window.close();
                            let _ = main_window.show();
                        }
                        Err(e) => {
                            dialog::message(Some(&window), "Error", e.to_string());
                        }
                    }
                }
                Err(e) => {
                    emit_initialize_event(&window, Response::fail(e.to_string().as_str(), "")).ok();
                }
            };
        });
    });
}

fn import_series(window: &Window, db: PathBuf) -> Result<()> {
    let yyets_connection = connect(&db)?;

    let total_size = yyets_connection.query_row("select count(*) as c from yyets", [], |row| {
        let size = row.get::<&str, i32>("c")?;
        Ok(size)
    })?;
    if total_size <= 0 {
        let _ = remove_file(db);
        return Ok(());
    }

    database::execute(|connection| {
        let mut statement = yyets_connection.prepare("select * from yyets")?;
        let mut result = statement.query([])?;
        let mut saved = 0;
        let mut areas: Vec<NameOnly> = vec![];
        let mut channels: Vec<NameOnly> = vec![];
        let mut ways: Vec<NameOnly> = vec![];
        let trx = connection.transaction()?;
        while let Some(row) = result.next()? {
            let _ = Ok(()).and_then(|()| {
                match row.get::<&str, String>("data") {
                    Ok(data) => Ok(data),
                    Err(e) => Err(anyhow::Error::new(e))
                }
            })
            .and_then(|data| {
                let record = serde_json::from_str::<RecordData>(data.as_str())?;
                Ok(record)
            })
            .and_then(|record| {
                let info = record.data.info;
                let channel_id = get_id_by_name(&trx, "channels", info.channel, &mut channels);
                let area_id = get_id_by_name(&trx, "areas", info.area, &mut areas);
                let size = trx.execute(
                "insert into resources (name, original_name, alias_name, channel_id, area_id) values (?1, ?2, ?3, ?4, ?5)",
                    params![info.cnname, info.enname, info.aliasname, channel_id, area_id],
                )?;
                Ok((size, record.data.list))
            })
            .and_then(|(size, seasons)| {
                if size <= 0 {
                    return Ok(());
                }
                let resource_id = trx.last_insert_rowid();
                insert_seasons(&trx, resource_id, seasons, &mut ways)?;
                Ok(())
            });

            saved += 1;
            emit_initialize_event(
                window,
                Response::ok(
                    "导入数据",
                    ResponseData::new("importing", (saved, total_size)),
                ),
            )
            .ok();
        }
        trx.commit()?;
        let _ = remove_file(db);
        Ok(())
    })
}

fn insert_seasons(
    trx: &Transaction,
    resource_id: i64,
    seasons: Vec<YYeTsSeason>,
    ways: &mut Vec<NameOnly>,
) -> Result<()> {
    for season in seasons {
        let _ = trx
            .execute(
                "insert into seasons (resource_id, season, name) values (?1, ?2, ?3)",
                params![resource_id, season.season_num, season.season_cn],
            )
            .and_then(|size| {
                if size > 0 {
                    let season_id = trx.last_insert_rowid();
                    let _ = insert_formats(trx, season_id, season.formats, season.items, ways);
                }
                Ok(())
            });
    }
    Ok(())
}

fn insert_formats(
    trx: &Transaction,
    season_id: i64,
    formats: Vec<String>,
    items: HashMap<String, Vec<Item>>,
    ways: &mut Vec<NameOnly>,
) -> Result<()> {
    for format in formats {
        if let Ok(size) = trx.execute(
            "insert into formats (season_id, format) values (?1, ?2)",
            params![season_id, format],
        ) {
            if size > 0 {
                if let Some(series) = items.get(format.as_str()) {
                    let format_id = trx.last_insert_rowid();
                    for item in series {
                        if let Ok(size) = trx.execute(
                        "insert into series (format_id, episode, name, size) values (?1, ?2, ?3, ?4)",
                            params![format_id, item.episode, item.name, item.size],
                        ) {
                            if size > 0 {
                                if let Some(files) = &item.files {
                                    let series_id = trx.last_insert_rowid();
                                    for file in files {
                                        let way_id = get_id_by_name(trx, "ways", file.way_cn.clone(), ways);
                                        let _ = trx.execute(
                                            "insert into files (series_id, way_id, address, password) values (?1, ?2, ?3, ?4)",
                                            params![series_id, way_id, file.address, file.passwd]
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn get_id_by_name(
    trx: &Transaction,
    table: &str,
    item: String,
    collection: &mut Vec<NameOnly>,
) -> i64 {
    let name = item.trim();
    match collection.iter().find(|record| record.name.trim().eq(name)) {
        Some(record) => record.id,
        None => {
            match trx.execute(
                format!("insert into {} (name) values (?1)", table).as_str(),
                params![name],
            ) {
                Ok(size) => {
                    let mut id = 0;
                    if size > 0 {
                        id = trx.last_insert_rowid();
                        collection.push(NameOnly {
                            id,
                            name: name.to_string(),
                        });
                    }
                    id
                }
                Err(_) => 0,
            }
        }
    }
}

pub fn emit_initialize_event<T: serde::ser::Serialize>(
    window: &Window,
    response: Response<T>,
) -> Result<(), tauri::Error> {
    window.emit("rubick_initialize", &response)
}

pub fn clear() {
    let _ = execute(|connection| {
        let _ = connection.execute_batch(
            "delete from resources;
            delete from areas;
            delete from channels;
            delete from files;
            delete from formats;
            delete from resources;
            delete from seasons;
            delete from series;
            delete from ways;
            delete from sqlite_sequence where name in ('resources', 'areas', 'channels', 'files', 'formats', 'resources', 'seasons', 'series', 'ways')",
        );
        Ok(())
    });
}
