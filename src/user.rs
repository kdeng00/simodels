use std::default::Default;

use crate::init;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct User {
    #[serde(skip_serializing_if = "init::is_uuid_nil")]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub phone: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub firstname: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub lastname: String,
    pub email_verified: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_created: Option<time::OffsetDateTime>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<time::OffsetDateTime>,
    #[serde(skip_serializing_if = "init::is_uuid_nil")]
    pub salt_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct UserProfile {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String
}

impl Default for User {
    fn default() -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            username: String::new(),
            password: String::new(),
            email: String::new(),
            phone: String::new(),
            firstname: String::new(),
            lastname: String::new(),
            email_verified: false,
            date_created: None,
            status: String::new(),
            last_login: None,
            salt_id: uuid::Uuid::nil(),
        }
    }
}

impl User {
    pub fn to_json(&self, output_pretty: bool) -> Result<String, serde_json::Error> {
        if output_pretty {
            serde_json::to_string_pretty(&self)
        } else {
            serde_json::to_string(&self)
        }
    }
}

pub mod salt {
    use std::default::Default;

    use crate::init;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Salt {
        #[serde(skip_serializing_if = "init::is_uuid_nil")]
        pub id: uuid::Uuid,
        #[serde(skip_serializing_if = "String::is_empty")]
        pub salt: String,
    }

    impl Salt {
        pub fn to_json(&self, output_pretty: bool) -> Result<String, serde_json::Error> {
            if output_pretty {
                serde_json::to_string_pretty(&self)
            } else {
                serde_json::to_string(&self)
            }
        }
    }
}
