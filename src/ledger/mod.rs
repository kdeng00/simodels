
pub struct SongEvent {
    pub id: uuid::Uuid,
    pub song_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub event_type: String,
    pub event_time: time::OffsetDateTime
}

pub const SONG_EVENT_TYPE_PLAY: &str = "PLAY";
pub const SONG_EVENT_TYPE_PAUSE: &str = "PAUSE";
