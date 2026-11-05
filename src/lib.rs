pub mod access_level;
pub mod album;
pub mod constants;
pub mod coverart;
pub mod ledger;
pub mod login_result;
pub mod song;
pub mod token;
pub mod types;
pub mod user;
pub mod util;

pub mod init {
    pub fn is_id_valid(num: &i32) -> bool {
        *num > 0
    }

    pub fn is_uuid_nil(uuid: &uuid::Uuid) -> bool {
        uuid.is_nil()
    }

    pub fn is_zero(num: &i32) -> bool {
        *num == 0
    }

    pub fn is_dur_not_set(num: &i32) -> bool {
        *num == 0
    }

    pub fn is_set(num: &i32) -> bool {
        *num >= 0
    }
}
