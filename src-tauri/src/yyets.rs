use anyhow::Result;
use async_std::prelude::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    error::Error,
    fmt,
    fs::File,
    hash::Hasher,
    io::{copy, Cursor, Read, Seek, Write},
    path::PathBuf,
    time,
};
use tauri::{api::path, Window};
use zip::ZipArchive;

use crate::{
    initialize::{emit_initialize_event, ResponseData},
    response::Response,
};

const DOWNLOAD_URL: &str = "https://yyets.dmesg.app/dump/yyets_sqlite.zip";

struct YYeTsError {
    error: Box<dyn Error + Send + Sync>,
}

impl fmt::Debug for YYeTsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.error.to_string().as_str(), f)
    }
}

impl fmt::Display for YYeTsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.error.to_string().as_str())
    }
}

impl YYeTsError {
    pub fn new<E: Into<Box<dyn Error + Send + Sync>>>(error: E) -> Self {
        YYeTsError {
            error: error.into(),
        }
    }
}

impl Error for YYeTsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub id: i32,
    pub cnname: String,
    pub enname: String,
    pub aliasname: String,
    pub channel: String,
    pub channel_cn: String,
    pub area: String,
    pub show_type: String,
    pub expire: String,
    pub views: i32,
    pub year: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct YYeTsSeason {
    pub season_num: String,
    pub season_cn: String,
    pub items: HashMap<String, Vec<Item>>,
    pub formats: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub itemid: String,
    pub episode: String,
    pub name: String,
    pub size: String,
    pub yyets_trans: i32,
    pub dateline: String,
    pub files: Option<Vec<YYeTsFile>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YYeTsFile {
    pub way: String,
    pub way_cn: String,
    pub address: String,
    pub passwd: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub info: Info,
    pub list: Vec<YYeTsSeason>,
}

#[derive(Serialize, Deserialize)]
pub struct RecordData {
    pub status: i32,
    pub info: String,
    pub data: Data,
}

pub async fn download(window: Window) -> Result<PathBuf> {
    download_database(window.clone()).await.and_then(unzip)
}

fn tmp_path(file: &str) -> Result<PathBuf> {
    let cache_dir = path::cache_dir().ok_or(YYeTsError::new("Cannot get cache dir"))?;
    Ok(cache_dir.join(file))
}

fn tmp_file(path: &PathBuf) -> Result<File> {
    if path.exists() {
        let file = File::open(path)?;
        Ok(file)
    } else {
        let file = File::create(path)?;
        Ok(file)
    }
}

async fn download_database(window: Window) -> Result<Cursor<Vec<u8>>> {
    let response = reqwest::get(DOWNLOAD_URL).await?;
    let total_size = response
        .content_length()
        .ok_or(YYeTsError::new("Failed to download yyets database"))?;

    let mut stream = response.bytes_stream();
    let mut buf = Cursor::new(Vec::new());
    let mut downloaded: u64 = 0;
    while let Some(item) = stream.next().await {
        let chunk = item?;
        buf.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        emit_initialize_event(
            &window,
            Response::ok(
                "下载人人影视数据库",
                ResponseData::new("downloading", (downloaded, total_size)),
            ),
        )
        .ok();
    }
    Ok(buf)
}

fn unzip(reader: impl Read + Seek) -> Result<PathBuf> {
    let mut random_db = random_string()?;
    random_db.push_str(".db");
    let mut archive = ZipArchive::new(reader)?;
    let mut zip_file = archive.by_name("yyets_sqlite.db")?;
    let db = tmp_path(&random_db)?;
    let mut out_file = tmp_file(&db)?;
    copy(&mut zip_file, &mut out_file)?;
    Ok(db)
}

fn random_string() -> Result<String> {
    let mut hasher = DefaultHasher::new();
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)?
        .as_nanos();
    hasher.write_u128(now);
    Ok(hasher.finish().to_string())
}
