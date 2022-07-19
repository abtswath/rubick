use anyhow::Result;
use rusqlite::{Connection, Row};
use std::path::PathBuf;

use crate::application::{app_dir};

const MIGRATIONS: [&'static str; 15] = [
    "CREATE TABLE IF NOT EXISTS areas (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        name text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS channels (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        name text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS files (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        series_id integer NOT NULL DEFAULT 0,
        way_id integer NOT NULL DEFAULT 0,
        address text NOT NULL DEFAULT '',
        password text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS formats (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        season_id integer NOT NULL DEFAULT 0,
        format text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS resources (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        name text NOT NULL DEFAULT '',
        original_name text NOT NULL DEFAULT '',
        alias_name text NOT NULL DEFAULT '',
        channel_id integer NOT NULL DEFAULT 0,
        area_id integer NOT NULL DEFAULT 0,
        directors text NOT NULL DEFAULT '',
        writers text NOT NULL DEFAULT '',
        actors text NOT NULL DEFAULT '',
        types text NOT NULL DEFAULT '',
        released_at text NOT NULL DEFAULT '',
        summary text NOT NULL DEFAULT '',
        rating real NOT NULL DEFAULT 0
    );",
    "CREATE TABLE IF NOT EXISTS seasons (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        resource_id integer NOT NULL DEFAULT 0,
        season integer NOT NULL DEFAULT 0,
        name text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS series (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        format_id integer NOT NULL DEFAULT 0,
        episode integer NOT NULL DEFAULT 0,
        name text NOT NULL DEFAULT '',
        size text NOT NULL DEFAULT ''
    );",
    "CREATE TABLE IF NOT EXISTS ways (
        id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
        name text NOT NULL DEFAULT ''
    );",
    "CREATE INDEX IF NOT EXISTS series_id ON files (
        series_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS way_id ON files (
        way_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS season_id ON formats (
        season_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS area_id ON resources (
        area_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS channel_id ON resources (
        channel_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS resource_id ON seasons (
        resource_id ASC
    );",
    "CREATE INDEX IF NOT EXISTS format_id ON series (
        format_id ASC
    );",
];

pub trait Model {
    fn from_row(row: &Row) -> Result<Self>
    where
        Self: Sized;
}

pub fn connect(db: &PathBuf) -> Result<Connection> {
    let connection = Connection::open(db)?;
    rusqlite::vtab::array::load_module(&connection)?;
    Ok(connection)
}

pub fn migrate(connection: &Connection) -> Result<()> {
    for migration in MIGRATIONS {
        connection.execute(migration, [])?;
    }
    Ok(())
}

pub fn db() -> PathBuf {
    app_dir().join("rubick.db")
}

pub fn setup() -> Result<()> {
    let db = db();
    let connection = connect(&db)?;
    migrate(&connection)?;
    Ok(())
}

pub fn execute<T, F>(f: F) -> Result<T>
where
    F: FnOnce(&mut Connection) -> Result<T>,
{
    match connect(&db()) {
        Ok(mut connection) => f(&mut connection),
        Err(err) => Err(err),
    }
}
