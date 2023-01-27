pub use rusqlite::{params, Connection};

pub mod port;

use crate::configuration::port::Port;
use crate::db;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub api: ApiConfiguration,
    pub local_port: Option<Port>,
}

#[derive(Debug, Clone)]
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
            local_port: None,
        }
    }

    pub fn init(db: &Connection) -> Result<Self, Box<dyn std::error::Error>> {
        // Select the config from the database
        let mut stmt = db.prepare(db::SELECT_CONFIG).unwrap();
        let config: Configuration = stmt
            .query_map(params![], |row| {
                Ok(Configuration {
                    api: ApiConfiguration {
                        id: row.get(0).ok(),
                        secret: row.get(1).ok(),
                        auth_url: row.get(2).ok(),
                        token_url: row.get(3).ok(),
                    },
                    local_port: row.get::<_, u16>(4).map(|u| u.into()).ok(),
                })
                // In order: propagate query errors,
                // default a new Configuration if empty table,
                // and propagate any mapping errors
            })?
            .next()
            .unwrap_or(Ok(Configuration::new()))?;

        Ok(config)
    }

    pub fn selective_immutable_update(
        &self,
        id: Option<String>,
        secret: Option<String>,
        auth_url: Option<String>,
        token_url: Option<String>,
        local_port: Option<Port>,
        db: &Connection,
    ) -> Result<Configuration, Error> {
        let new_config = Self {
            api: ApiConfiguration {
                id: id.or(self.api.id.clone()),
                secret: secret.or(self.api.secret.clone()),
                auth_url: auth_url.or(self.api.auth_url.clone()),
                token_url: token_url.or(self.api.token_url.clone()),
            },
            local_port: local_port.or(self.local_port),
        };

        db.execute(
            "UPDATE config SET api_id = ?, api_secret = ?, auth_url = ?, token_url = ?, local_port = ? WHERE id = 1",
            params![new_config.api.id, new_config.api.secret, new_config.api.auth_url, new_config.api.token_url.clone(), new_config.local_port.map(|p| p.as_u16())]
        ).map_err(Box::from)?;

        Ok(new_config)
    }

    pub fn update_config(
        &mut self,
        id: Option<String>,
        secret: Option<String>,
        auth_url: Option<String>,
        token_url: Option<String>,
        local_port: Option<Port>,
        db: &Connection,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        db.execute(
            "UPDATE config SET api_id = ?, api_secret = ?, auth_url = ?, token_url = ?, local_port = ? WHERE id = 1",
            params![id, secret, auth_url, token_url, local_port.map(|p| p.as_u16())]
        )?;

        Ok(Self {
            api: ApiConfiguration {
                id,
                secret,
                auth_url,
                token_url,
            },
            local_port: local_port.map(|u| u.into()),
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
        self.id.is_some()
            && self.secret.is_some()
            && self.auth_url.is_some()
            && self.token_url.is_some()
    }
}
