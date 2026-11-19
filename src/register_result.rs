use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RegisterResult {
    pub id: uuid::Uuid,
    pub username: String,
    pub date_created: Option<time::OffsetDateTime>
}
