use std::default::Default;

use serde::{Deserialize, Serialize};

const DEFAULT_ACCESS_LEVEL: &str = super::constants::access_level::PUBLIC;

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct AccessLevel {
    pub id: uuid::Uuid,
    pub level: String,
    pub song_id: uuid::Uuid,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel {
            id: uuid::Uuid::nil(),
            level: DEFAULT_ACCESS_LEVEL.to_string(),
            song_id: uuid::Uuid::nil(),
        }
    }
}

pub fn default_level() -> AccessLevel {
    AccessLevel {
        id: uuid::Uuid::nil(),
        level: DEFAULT_ACCESS_LEVEL.to_string(),
        song_id: uuid::Uuid::nil(),
    }
}

pub fn private_level() -> AccessLevel {
    AccessLevel {
        id: uuid::Uuid::nil(),
        level: super::constants::access_level::PRIVATE.to_string(),
        song_id: uuid::Uuid::nil(),
    }
}

impl AccessLevel {
    pub fn _to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
