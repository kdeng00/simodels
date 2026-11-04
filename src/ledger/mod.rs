#[derive(Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SongEvent {
    pub id: uuid::Uuid,
    pub song_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub event_type: String,
    pub event_time: Option<time::OffsetDateTime>,
    pub date_created: Option<time::OffsetDateTime>,
}

impl SongEvent {
    pub fn is_event_type_valid(&self) -> bool {
        matches!(
            self.event_type.as_str(),
            SONG_EVENT_TYPE_PLAY | SONG_EVENT_TYPE_PAUSE
        )
    }
}

/// Song Event Type - Play
pub const SONG_EVENT_TYPE_PLAY: &str = "PLAY";
/// Song Event Type - PAUSE
pub const SONG_EVENT_TYPE_PAUSE: &str = "PAUSE";
