pub use rusqlite::{params, Connection};

use crate::db;

#[derive(Debug)]
pub struct Configuration {
    pub api: ApiConfiguration,
}

#[derive(Debug)]
pub struct ApiConfiguration {
    pub id: Option<String>,
    pub secret: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
}

impl Configuration {

    fn new() -> Self {
        Self {
            api: ApiConfiguration::new(),
        }
    }

    pub fn init(db: &Connection) -> Result<Self, Box<dyn std::error::Error>> {

        // Select the config from the database
        let mut stmt = db.prepare(db::SELECT_CONFIG).unwrap();
        let api: ApiConfiguration = stmt.query_map(params![], |row| {
            Ok(ApiConfiguration {
                id: row.get(0).ok(),
                secret: row.get(1).ok(),
                auth_url: row.get(2).ok(),
                token_url: row.get(3).ok(),
            })
        // In order: propagate query errors, 
        // default a new ApiConfiguration if empty table, 
        // and propagate any mapping errors
        })?.next().unwrap_or(Ok(ApiConfiguration::new()))?;

        Ok(Self {
            api,
        })
    }

    pub fn update_api(
        &mut self,
        id: Option<String>,
        secret: Option<String>,
        auth_url: Option<String>,
        token_url: Option<String>,
        db: &Connection
    ) -> Result<Self, Box<dyn std::error::Error>> {

        //Save back to DB
        db.execute(
            "UPDATE config SET api_id = ?, api_secret = ?, auth_url = ?, token_url = ? WHERE id = 1",
            params![id, secret, auth_url, token_url]
        )?;

        Ok(Self {
            api: ApiConfiguration {
                id,
                secret,
                auth_url,
                token_url,
            }
        })
    }

    pub fn is_complete(&self) -> bool {
        self.api.is_complete()
    }

    pub fn reset(db: &Connection) -> Result<Self, Box<dyn std::error::Error>> {
        db::reset_config(db)?;
        Ok(Self::new())
    }
}

impl ApiConfiguration {
    fn new() -> Self {
        Self {
            id: None,
            secret: None,
            auth_url: None,
            token_url: None,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.id.is_some() &&
        self.secret.is_some() &&
        self.auth_url.is_some() &&
        self.token_url.is_some()
    }

    // Checks a string for length > 0
    fn has_length(s: &String) -> bool {
        s.len() > 0
    }

}