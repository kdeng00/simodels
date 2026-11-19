use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RegisterResult {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_created: Option<time::OffsetDateTime>,
}
