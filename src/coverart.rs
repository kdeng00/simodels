use std::io::Write;

use rand::RngExt;

const FILENAME_LENGTH: i32 = 16;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CoverArt {
    pub id: uuid::Uuid,
    pub title: String,
    #[serde(skip)]
    pub directory: String,
    pub filename: String,
    pub file_type: String,
    #[serde(skip)]
    pub data: Vec<u8>,
    pub song_id: uuid::Uuid,
}

pub mod init {
    /// Initializes the CoverArt with just the directory and filename
    pub fn init_coverart_dir_and_filename(directory: &str, filename: &str) -> super::CoverArt {
        super::CoverArt {
            directory: String::from(directory),
            filename: String::from(filename),
            ..Default::default()
        }
    }
}

impl CoverArt {
    /// Saves the coverart to the filesystem
    pub fn save_to_filesystem(&self) -> Result<(), std::io::Error> {
        match self.get_path() {
            Ok(path) => match std::fs::File::create(&path) {
                Ok(mut file) => match file.write_all(&self.data) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                },
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    /// Removes the coverart from the filesystem
    pub fn remove_from_filesystem(&self) -> Result<(), std::io::Error> {
        match self.get_path() {
            Ok(path) => {
                let p = std::path::Path::new(&path);
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

    /// Gets the path of the CoverArt
    pub fn get_path(&self) -> Result<String, std::io::Error> {
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
}

/// Generates filename for a CoverArt
pub fn generate_filename(
    typ: crate::types::CoverArtType,
    randomize: bool,
) -> Result<String, std::io::Error> {
    let file_extension = match typ {
        crate::types::CoverArtType::PngExtension => {
            String::from(crate::constants::file_extensions::image::PNGEXTENSION)
        }
        crate::types::CoverArtType::JpegExtension => {
            String::from(crate::constants::file_extensions::image::JPEGEXTENSION)
        }
        crate::types::CoverArtType::JpgExtension => {
            String::from(crate::constants::file_extensions::image::JPGEXTENSION)
        }
        crate::types::CoverArtType::None => {
            return Err(std::io::Error::other("Unsupported CoverArtTypes"));
        }
    };

    let filename: String = if randomize {
        let mut filename: String = String::from("coverart-");
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
        format!("coverart-output{file_extension}")
    };

    Ok(filename)
}

pub mod io {
    use std::io::Read;

    /// Gets the raw data of the cover art
    pub fn to_data(coverart: &super::CoverArt) -> Result<Vec<u8>, std::io::Error> {
        match coverart.get_path() {
            Ok(path) => {
                let mut file = std::fs::File::open(path)?;
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(_) => Ok(buffer),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coverart;

    #[test]
    fn test_cover_art_image() {
        let dir = String::from("./");
        let filename = String::from("CoverArt.png");
        let coverart = coverart::init::init_coverart_dir_and_filename(&dir, &filename);

        assert_eq!(dir, coverart.directory);
        assert_eq!(filename, coverart.filename);
    }
}
