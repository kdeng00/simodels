use std::io::Write;

use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::constants;
use crate::init;
use crate::types;

/// Length of characters of a filename to be generated
const FILENAME_LENGTH: i32 = 16;

#[derive(Clone, Debug, Default, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Song {
    #[serde(skip_serializing_if = "init::is_uuid_nil")]
    #[serde(alias = "id")]
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub artist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub album: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub album_artist: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub genre: String,
    #[serde(skip_serializing_if = "init::is_zero")]
    pub year: i32,
    #[serde(skip_serializing_if = "init::is_dur_not_set")]
    pub duration: i32,
    #[serde(skip_serializing_if = "init::is_zero")]
    pub track: i32,
    #[serde(skip_serializing_if = "init::is_zero")]
    pub disc: i32,
    #[serde(skip_serializing_if = "init::is_zero")]
    pub disc_count: i32,
    #[serde(skip_serializing_if = "init::is_zero")]
    pub track_count: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub audio_type: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub date_created: Option<time::OffsetDateTime>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub filename: String,
    #[serde(skip_serializing_if = "init::is_uuid_nil")]
    pub user_id: uuid::Uuid,
    #[serde(skip)]
    pub data: Vec<u8>,
    #[serde(skip)]
    pub directory: String,
    // TODO: Think about what to do with this
    // #[serde(skip)]
    // pub album_id: i32,
    // #[serde(skip)]
    // pub artist_id: i32,
    // #[serde(skip)]
    // pub genre_id: i32,
    // #[serde(skip)]
    // pub coverart_id: i32,
}

impl Song {
    pub fn to_metadata_json(&self, pretty: bool) -> Result<String, serde_json::Error> {
        if pretty {
            serde_json::to_string_pretty(&self)
        } else {
            serde_json::to_string(&self)
        }
    }

    /// Gets the path of a Song
    pub fn song_path(&self) -> Result<String, std::io::Error> {
        if self.directory.is_empty() {
            return Err(std::io::Error::other(
                crate::constants::error::DIRECTORY_NOT_INITIALIZED,
            ));
        } else if self.filename.is_empty() {
            return Err(std::io::Error::other(
                crate::constants::error::FILENAME_NOT_INITIALIZED,
            ));
        }

        let directory = &self.directory;
        let last_index = directory.len() - 1;

        match crate::util::concatenate_path(directory, &self.filename, last_index) {
            Ok(path) => Ok(path),
            Err(err) => Err(err),
        }
    }

    /// Saves the song to the filesystem using the song's data
    pub fn save_to_filesystem(&self) -> Result<(), std::io::Error> {
        match self.song_path() {
            Ok(song_path) => match std::fs::File::create(&song_path) {
                Ok(mut file) => match file.write_all(&self.data) {
                    Ok(_res) => Ok(()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    /// Removes the song from the filesystem
    pub fn remove_from_filesystem(&self) -> Result<(), std::io::Error> {
        match self.song_path() {
            Ok(song_path) => {
                let p = std::path::Path::new(&song_path);
                if p.exists() {
                    match std::fs::remove_file(p) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(std::io::Error::other(
                        "Cannot delete file that does not exist",
                    ))
                }
            }
            Err(err) => Err(err),
        }
    }
}

/// Generates a filename. In order to save a song to the filesystem
pub fn generate_filename(typ: types::MusicType, randomize: bool) -> Result<String, std::io::Error> {
    let file_extension = match typ {
        types::MusicType::DefaultMusicExtension => {
            String::from(constants::file_extensions::audio::DEFAULTMUSICEXTENSION)
        }
        types::MusicType::WavExtension => {
            String::from(constants::file_extensions::audio::WAVEXTENSION)
        }
        types::MusicType::FlacExtension => {
            String::from(constants::file_extensions::audio::FLACEXTENSION)
        }
        types::MusicType::MPThreeExtension => {
            String::from(constants::file_extensions::audio::MPTHREEEXTENSION)
        }
        types::MusicType::None => return Err(std::io::Error::other("Unsupported MusicTypes")),
    };

    let filename: String = if randomize {
        let mut filename: String = String::from("track-");
        let some_chars: String = String::from("abcdefghij0123456789");
        let some_chars_length = some_chars.len();
        let mut rng = rand::rng();

        for _ in 0..FILENAME_LENGTH {
            let index = rng.random_range(0..=some_chars_length);
            let rando_char = some_chars.chars().nth(index);

            if let Some(c) = rando_char {
                filename.push(c);
            }
        }
        format!("{filename}{file_extension}")
    } else {
        format!("track-output{file_extension}")
    };

    Ok(filename)
}

/// I/O operations for songs
pub mod io {
    use std::io::Read;

    /// Copies a song using the source song's data
    pub fn copy_song(
        song_source: &super::Song,
        song_target: &mut super::Song,
    ) -> Result<(), std::io::Error> {
        match song_target.song_path() {
            Ok(songpath) => {
                let p = std::path::Path::new(&songpath);
                if p.exists() {
                    Err(std::io::Error::other(
                        "Cannot copy song over to one that already exists",
                    ))
                } else {
                    if song_target.data.is_empty() {
                        song_target.data = song_source.data.clone();
                    } else {
                        song_target.data.clear();
                        song_target.data = song_source.data.clone();
                    }

                    match song_target.save_to_filesystem() {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Gets the raw file data of a song from the filesystem
    pub fn to_data(song: &super::Song) -> Result<Vec<u8>, std::io::Error> {
        match song.song_path() {
            Ok(path) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer)?;

                if buffer.is_empty() {
                    Err(std::io::Error::other("File is empty"))
                } else {
                    Ok(buffer)
                }
            }
            Err(er) => Err(er),
        }
    }
}
