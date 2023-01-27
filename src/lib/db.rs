pub use rusqlite::{params, Connection};
use std::{fmt::Display};
use crate::error::Error;

/// A sanitized, whitelisted set of fields that we can
/// plug into queries.
pub enum EditableConfigFields {
    ApiId,
    ApiSecret,
    AuthUrl,
    TokenUrl,
    LocalPort
}

impl Display for EditableConfigFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ApiId => write!(f, "api_id"),
            Self::ApiSecret => write!(f, "api_secret"),
            Self::AuthUrl => write!(f, "auth_url"),
            Self::TokenUrl => write!(f, "token_url"),
            Self::LocalPort => write!(f, "local_port"),
        }
    }
}

// Initializes sqlite3 database, creates config table if it doesn't exist.
pub fn init() -> Result<Connection, Error> {
    let db = open()?;
    db.execute(
        "CREATE TABLE IF NOT EXISTS config (
            id              INTEGER PRIMARY KEY,
            api_id          TEXT,
            api_secret      TEXT,
            auth_url        TEXT,
            token_url       TEXT,
            local_port      INTEGER
        );",
        params![],
    ).map_err(Box::from)?;

    db.execute("DROP TABLE IF EXISTS labels;", params![]).map_err(Box::from)?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS labels (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            postcard        BLOB
        );",
        params![],
    ).map_err(Box::from)?;

    db.execute("INSERT OR IGNORE INTO config (id) VALUES (1)", params![]).map_err(Box::from)?;

    Ok(db)
}

pub fn reset_config(db: &Connection) -> Result<(), Error> {
    db.execute("REPLACE INTO config (id) VALUES (1)", params![]).map_err(Box::from)?;
    Ok(())
}

pub fn update_config<T: rusqlite::ToSql>(field: EditableConfigFields, value: T, db: &Connection) -> Result<(), Error> {
    db.prepare(format!("UPDATE config SET {} = ? WHERE id = 1", field).as_str()).map_err(Box::from)?
        .execute(params![value]).map_err(Box::from)?;

    Ok(())
}

pub fn update_full_config(
    id: Option<String>,
    secret: Option<String>,
    auth_url: Option<String>,
    token_url: Option<String>,
    local_port: Option<u16>,
    db: &Connection,
) -> Result<(), Error> {
    db.execute(
        "UPDATE config SET api_id = ?, api_secret = ?, auth_url = ?, token_url = ?, local_port = ? WHERE id = 1",
        params![id, secret, auth_url, token_url, local_port]
    ).map_err(Box::from)?;

    Ok(())
}

pub const SELECT_CONFIG: &str =
    "SELECT api_id, api_secret, auth_url, token_url, local_port FROM config WHERE id = 1";

fn open() -> Result<Connection, Error> {
    let db = Connection::open("rust_requester.db").map_err(Box::from)?;
    Ok(db)
}

pub fn get_labels(db: &Connection) -> Vec<Result<String, rusqlite::Error>> {
    db.prepare("SELECT name FROM labels")
        .expect("Failed to prepare query.")
        .query_map([], |row| {
            let name: String = row.get(0).unwrap_or("UNNAMED".to_owned());
            Ok(name)
        })
        .expect("Failed to query labels.")
        .collect()
}
