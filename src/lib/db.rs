pub use rusqlite::{Connection, params};
use std::error::Error;

// Initializes sqlite3 database, creates config table if it doesn't exist.
pub fn init() -> Result<Connection, Box<dyn Error>> {
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
    )?;

    db.execute("DROP TABLE IF EXISTS labels;", params![])?;

    db.execute(
        "CREATE TABLE IF NOT EXISTS labels (
            id              INTEGER PRIMARY KEY,
            name            TEXT,
            postcard        BLOB
        );", params![]
    )?;

    db.execute(
        "INSERT OR IGNORE INTO config (id) VALUES (1)",
        params![],
    )?;

    Ok(db)
}

pub fn reset_config(db: &Connection) -> Result<(), Box<dyn Error>> {
    db.execute("REPLACE INTO config (id) VALUES (1)", params![])?;
    Ok(())
}

pub const SELECT_CONFIG: &str = "SELECT api_id, api_secret, auth_url, token_url, local_port FROM config WHERE id = 1";

fn open() -> Result<Connection, Box<dyn Error>> {
    let db = Connection::open("rust_requester.db")?;
    Ok(db)
}