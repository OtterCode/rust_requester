use rusqlite::{params, Connection};

use crate::db;

#[derive(Debug)]
pub struct  Configuration {
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

    pub fn init(db: &Connection) -> Result<Self, Box<dyn std::error::Error>> {

        // Select the config from the database
        let mut stmt = db.prepare(db::SELECT_CONFIG).unwrap();
        let db_api: ApiConfiguration = stmt.query_map(params![], |row| {
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

        let api = if !db_api.is_complete() {
            db_api.fill(&db)
        } else {
            db_api
        };

        Ok(Self {
            api,
        })
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

    fn is_complete(&self) -> bool {
        self.id.is_some() &&
        self.secret.is_some() &&
        self.auth_url.is_some() &&
        self.token_url.is_some()
    }

    // Checks a string for length > 0
    fn has_length(s: &String) -> bool {
        s.len() > 0
    }

    fn fill(self, db: &Connection) -> Self {
        println!("Please enter missing API credentials");
        let mut rl = rustyline::Editor::<()>::new().unwrap();
        let api_id = if self.id.is_some() {
            self.id
        } else {
            rl.readline("API ID: ").ok()
        }.filter(Self::has_length);

        let api_secret = if self.secret.is_some() {
            self.secret
        } else {
            rl.readline("API Secret: ").ok()
        }.filter(Self::has_length);

        let auth_url = if self.auth_url.is_some() {
            self.auth_url
        } else {
            rl.readline("Auth URL: ").ok()
        }.filter(Self::has_length);

        let token_url = if self.token_url.is_some() {
            self.token_url
        } else {
            rl.readline("Token URL: ").ok()
        }.filter(Self::has_length);

        //Save back to DB
        db.execute(
            "UPDATE config SET api_id = ?, api_secret = ?, auth_url = ?, token_url = ? WHERE id = 1",
            params![api_id.clone(), api_secret.clone(), auth_url.clone(), token_url.clone()]
        ).unwrap();

        Self {
            id: api_id,
            secret: api_secret,
            auth_url: auth_url,
            token_url: token_url,
        }
    }
}