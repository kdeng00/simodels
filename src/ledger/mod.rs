
pub struct SongEvent {
    pub id: uuid::Uuid,
    pub song_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub event_type: String,
    pub event_time: time::OffsetDateTime
}

impl SongEvent {
    pub fn is_event_type_valid(&self) -> bool {
        match self.event_type.as_str() {
            SONG_EVENT_TYPE_PLAY => {
                true
            },
            SONG_EVENT_TYPE_PAUSE => {
                true
            },
            _ => {
                false
            }
        }
    }
}

/// Song Event Type - Play
pub const SONG_EVENT_TYPE_PLAY: &str = "PLAY";
/// Song Event Type - PAUSE
pub const SONG_EVENT_TYPE_PAUSE: &str = "PAUSE";
