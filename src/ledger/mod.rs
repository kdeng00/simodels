
pub struct SongEvent {
    pub id: uuid::Uuid,
    pub song_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub event_type: String,
    pub event_time: time::OffsetDateTime
}
