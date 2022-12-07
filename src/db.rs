use rusqlite::{Connection, params};
use std::error::Error;

// Initializes sqlite3 database, creates config table if it doesn't exist.
pub fn init() -> Result<Connection, Box<dyn Error>> {
    let conn = open()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS config (
            id              INTEGER PRIMARY KEY,
            api_id          TEXT,
            api_secret      TEXT,
            auth_url        TEXT,
            token_url       TEXT
        );
        CREATE TABLE IF NOT EXISTS labels (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            postcard        BLOB,
        );
        ",
        params![],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO config (id) VALUES (1)",
        params![],
    )?;

    Ok(conn)
}

pub const SELECT_CONFIG: &str = "SELECT api_id, api_secret, auth_url, token_url FROM config WHERE id = 1";

fn open() -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open("rust_requester.db")?;
    Ok(conn)
}