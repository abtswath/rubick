use anyhow::Result;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tauri::command;

use crate::{
    database::execute,
    douban::{download_image, get_subject},
    response::Response,
};

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub alias_name: String,
    pub channel: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesFile {
    pub id: i64,
    pub series_id: i64,
    pub address: String,
    pub password: String,
    pub way: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: i64,
    pub format_id: i64,
    pub episode: i64,
    pub name: String,
    pub size: String,
    pub files: Vec<SeriesFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub id: i64,
    pub season_id: i64,
    pub format: String,
    pub series: Vec<Series>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: i64,
    pub season: i64,
    pub name: String,
    pub formats: Vec<Format>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub alias_name: String,
    pub pic: String,
    pub directors: String,
    pub writers: String,
    pub actors: String,
    pub types: String,
    pub released_at: String,
    pub summary: String,
    pub rating: f64,
    pub seasons: Vec<Season>,
    pub channel: String,
    pub area: String,
    pub favorite: bool,
}

#[command]
pub async fn search(keyword: String) -> Response<Vec<SearchResult>> {
    match execute(|db| {
        let mut stmt = db.prepare("select r.id, r.name, r.original_name, r.alias_name, c.name as channel from resources as r left join channels as c on c.id=r.channel_id where r.name like ?1 or r.alias_name like ?1 or r.original_name like ?1 order by r.released_at desc, r.id desc")?;
        let mut rows = stmt.query(params![format!("%{}%", keyword)])?;
        let mut resources: Vec<SearchResult> = vec![];
        while let Some(row) = rows.next()? {
            resources.push(SearchResult {
                id: row.get("id")?,
                name: row.get("name")?,
                original_name: row.get("original_name")?,
                alias_name: row.get("alias_name")?,
                channel: row.get("channel")?,
            })
        }
        Ok(resources)
    }) {
        Ok(resources) => Response::ok("success", resources),
        Err(_) => Response::ok("success", Vec::new()),
    }
}

#[command]
pub async fn resource(id: i64) -> Response<Option<Resource>> {
    if id < 1 {
        return Response::fail("resource is not exists.", None);
    }
    let result = execute(|db| {
        let mut resource = db.query_row("select r.id, r.name, r.original_name, r.alias_name, r.pic, r.directors, r.writers, r.actors, r.types, r.released_at, r.summary, r.rating, c.name as channel, a.name as area from resources as r left join channels as c on c.id=r.channel_id left join areas as a on a.id=r.area_id where r.id=?", [id], |row| {
            Ok(Resource {
                id: row.get("id")?,
                name: row.get("name")?,
                original_name: row.get("original_name")?,
                alias_name: row.get("alias_name")?,
                pic: row.get("pic")?,
                directors: row.get("directors")?,
                writers: row.get("writers")?,
                actors: row.get("actors")?,
                types: row.get("types")?,
                released_at: row.get("released_at")?,
                summary: row.get("summary")?,
                rating: row.get("rating")?,
                channel: row.get("channel")?,
                area: row.get("area")?,
                favorite: false,
                seasons: vec![],
            })
        })?;
        resource.favorite = is_favorite(db, id);

        resource.seasons = seasons_for_resource(db, id)?;
        Ok(resource)
    });
    if let Ok(mut resource) = result {
        if resource.pic.eq("") {
            let result = get_subject(resource.name.as_str()).await;
            if let Ok(subject) = result {
                if let Ok(pic) = download_image(subject.pic.as_str()).await {
                    resource.pic = pic;
                }

                resource.directors = subject.directors.to_string();
                resource.writers = subject.writers.to_string();
                resource.actors = subject.actors.to_string();
                resource.types = subject.types.to_string();
                resource.released_at = subject.released_at.to_string();
                resource.summary = subject.summary.to_string();
                resource.rating = subject.rating;
                let _ = execute(|db| update_resource(db, &resource));
            }
        }
        return Response::ok("success", Some(resource));
    }
    Response::fail("resource is not exists.", None)
}

fn update_resource(db: &mut Connection, resource: &Resource) -> Result<usize> {
    let size = db.execute("update resources set pic=?1, directors=?2, writers=?3, actors=?4, types=?5, released_at=?6, summary=?7, rating=?8 where id=?9", params![
        resource.pic,
        resource.directors,
        resource.writers,
        resource.actors,
        resource.types,
        resource.released_at,
        resource.summary,
        resource.rating,
        resource.id
    ])?;
    Ok(size)
}

fn seasons_for_resource(db: &mut Connection, resource_id: i64) -> Result<Vec<Season>> {
    let mut season_ids = vec![];
    let mut seasons = db
        .prepare("select * from seasons where resource_id=?1 order by season asc")
        .and_then(|mut stmt| {
            let mut rows = stmt.query(params![resource_id])?;
            let mut seasons: Vec<Season> = vec![];
            while let Some(row) = rows.next()? {
                let id = row.get("id")?;
                season_ids.push(id);
                seasons.push(Season {
                    id,
                    season: row.get("season")?,
                    name: row.get("name")?,
                    formats: vec![],
                })
            }
            Ok(seasons)
        })?;
    if let Ok(formats) = formats_for_seasons(db, season_ids) {
        for season in seasons.iter_mut() {
            season.formats = formats
                .clone()
                .into_iter()
                .filter(|format| format.season_id == season.id)
                .collect();
        }
    }
    Ok(seasons)
}

fn formats_for_seasons(db: &mut Connection, season_ids: Vec<i64>) -> Result<Vec<Format>> {
    if season_ids.len() <= 0 {
        return Ok(vec![]);
    }
    let mut format_ids = vec![];
    let mut formats = db
        .prepare("select * from formats where season_id in rarray(?)")
        .and_then(|mut stmt| {
            let mut rows = stmt.query(&[&std::rc::Rc::new(
                season_ids
                    .into_iter()
                    .map(|x| rusqlite::types::Value::from(x))
                    .collect::<Vec<rusqlite::types::Value>>(),
            )])?;
            let mut formats = vec![];
            while let Some(row) = rows.next()? {
                let id = row.get("id")?;
                format_ids.push(id);
                formats.push(Format {
                    id,
                    season_id: row.get("season_id")?,
                    format: row.get("format")?,
                    series: vec![],
                });
            }
            Ok(formats)
        })?;
    if let Ok(series) = series_for_formats(db, format_ids) {
        for format in formats.iter_mut() {
            format.series = series
                .clone()
                .into_iter()
                .filter(|item| item.format_id == format.id)
                .collect();
        }
    }
    Ok(formats)
}

fn series_for_formats(db: &mut Connection, format_ids: Vec<i64>) -> Result<Vec<Series>> {
    if format_ids.len() <= 0 {
        return Ok(vec![]);
    }
    let mut series_ids = vec![];
    let mut series = db
        .prepare("select * from series where format_id in rarray(?) order by episode asc")
        .and_then(|mut stmt| {
            let mut rows = stmt.query(&[&std::rc::Rc::new(
                format_ids
                    .into_iter()
                    .map(|x| rusqlite::types::Value::from(x))
                    .collect::<Vec<rusqlite::types::Value>>(),
            )])?;
            let mut series = vec![];
            while let Some(row) = rows.next()? {
                let id = row.get("id")?;
                series_ids.push(id);
                series.push(Series {
                    id,
                    format_id: row.get("format_id")?,
                    episode: row.get("episode")?,
                    name: row.get("name")?,
                    size: row.get("size")?,
                    files: vec![],
                })
            }
            Ok(series)
        })?;
    if let Ok(files) = files_for_series(db, series_ids) {
        for series_item in series.iter_mut() {
            series_item.files = files
                .clone()
                .into_iter()
                .filter(|file| file.series_id == series_item.id)
                .collect();
        }
    }
    Ok(series)
}

fn files_for_series(db: &Connection, series_ids: Vec<i64>) -> Result<Vec<SeriesFile>> {
    if series_ids.len() <= 0 {
        return Ok(vec![]);
    }
    let files = db
        .prepare("select f.*, w.name as way from files as f left join ways as w on w.id=f.way_id where f.series_id in rarray(?)")
        .and_then(|mut stmt| {
            let mut rows = stmt.query(&[&std::rc::Rc::new(
                series_ids
                    .into_iter()
                    .map(|x| rusqlite::types::Value::from(x))
                    .collect::<Vec<rusqlite::types::Value>>(),
            )])?;
            let mut files = vec![];
            while let Some(row) = rows.next()? {
                let id = row.get("id")?;
                files.push(SeriesFile {
                    id,
                    series_id: row.get("series_id")?,
                    address: row.get("address")?,
                    password: row.get("password")?,
                    way: row.get("way")?,
                });
            }
            Ok(files)
        })?;
    Ok(files)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Favorite {
    pub id: i64,
    pub name: String,
    pub original_name: String,
    pub alias_name: String,
    pub pic: String,
}

#[command]
pub async fn favorites() -> Response<Vec<Favorite>> {
    let result = execute(|db| {
        let mut stmt = db.prepare("select r.id, r.name, r.original_name, r.alias_name, r.pic from favorites as f left join resources as r on f.resource_id=r.id")?;
        let mut rows = stmt.query([])?;
        let mut resources = vec![];
        while let Some(row) = rows.next()? {
            resources.push(Favorite {
                id: row.get("id")?,
                name: row.get("name")?,
                original_name: row.get("original_name")?,
                alias_name: row.get("alias_name")?,
                pic: row.get("pic")?,
            });
        }
        Ok(resources)
    });
    match result {
        Ok(resources) => Response::ok("success", resources),
        Err(e) => Response::fail(e.to_string().as_str(), Vec::new()),
    }
}

fn is_favorite(db: &mut Connection, resource_id: i64) -> bool {
    if let Ok(size) = db.query_row(
        "select count(*) from favorites where resource_id=?1",
        [resource_id],
        |row| Ok(row.get::<usize, usize>(0)?),
    ) {
        return size > 0;
    }
    return false;
}

#[command]
pub async fn favorite(resource_id: i64) -> Response<()> {
    let result = execute(|db| {
        if !(is_favorite(db, resource_id)) {
            db.execute(
                "insert into favorites (resource_id) values (?1)",
                [resource_id],
            )?;
        }
        Ok(())
    });
    match result {
        Ok(()) => Response::ok("success", ()),
        Err(e) => Response::fail(e.to_string().as_str(), ()),
    }
}

#[command]
pub async fn un_favorite(resource_id: i64) -> Response<()> {
    let result = execute(|db| {
        db.execute("delete from favorites where resource_id=?1", [resource_id])?;
        Ok(())
    });
    match result {
        Ok(()) => Response::ok("success", ()),
        Err(e) => Response::fail(e.to_string().as_str(), ()),
    }
}
