mod utils {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    pub fn get_tests_directory() -> String {
        String::from(env!("CARGO_MANIFEST_DIR").to_owned() + "/tests/")
    }

    pub fn does_directory_exists(directory: &String) -> bool {
        let path = Path::new(directory);
        if let Ok(dir_i) = fs::metadata(path) {
            dir_i.is_dir()
        } else {
            false
        }
    }

    pub fn extract_data_from_file(filepath: &String) -> Result<Vec<u8>, std::io::Error> {
        match std::fs::File::open(filepath) {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
                let _ = file.read_to_end(&mut buffer);
                Ok(buffer)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod song_tests {
    use tempfile::tempdir;

    use crate::utils;
    use simodels::song;
    use simodels::types;

    #[test]
    fn test_song_to_data() {
        println!("Test");
        let some_val = true;

        println!("Checking if some_val is true");
        assert_eq!(true, some_val);

        println!("Getting track");
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        assert!(
            utils::does_directory_exists(&song.directory),
            "Directory does not exist"
        );

        println!("Directory: {}", song.directory);

        match song.song_path() {
            Ok(filepath) => match utils::extract_data_from_file(&filepath) {
                Ok(buffer) => {
                    assert_eq!(buffer.is_empty(), false);

                    match song::io::to_data(&song) {
                        Ok(song_data) => {
                            println!("Both files match");
                            assert_eq!(buffer, song_data);
                        }
                        Err(err) => {
                            assert!(false, "Error producing song data: {:?}", err);
                        }
                    };
                }
                Err(err) => {
                    assert!(false, "Failed to open file: {:?}", err);
                }
            },
            Err(err) => {
                assert!(false, "Could not get song path: {:?}", err);
            }
        }
    }

    #[test]
    fn test_song_path_check() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        assert!(
            utils::does_directory_exists(&song.directory),
            "Directory does not exist"
        );
    }

    #[test]
    fn test_song_generate_filename() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track01.flac");

        let mut song_cpy = song.clone();
        let temp_dir = tempdir().expect("Failed to create temp dir");
        song_cpy.directory = match temp_dir.path().to_str() {
            Some(s) => String::from(s),
            None => String::new(),
        };

        assert_eq!(song.directory.is_empty(), false);
        match song::generate_filename(types::MusicType::FlacExtension, true) {
            Ok(filename) => {
                song_cpy.filename = filename;
            }
            Err(err) => {
                assert!(false, "Error generatig filename: {err:?}");
            }
        };
        println!("Directory: {:?}", song_cpy.directory);
        println!("File to be created: {:?}", song_cpy.filename);

        match song::io::copy_song(&song, &mut song_cpy) {
            Ok(_) => {
                println!("Song copied");
            }
            Err(err) => {
                assert!(false, "Error copying song: Error: {err:?}")
            }
        }
    }

    #[test]
    fn test_save_song_to_filesystem_and_remove() {
        let mut song = song::Song::default();
        song.directory = utils::get_tests_directory();
        song.filename = String::from("track02.flac");

        let mut copied_song = song::Song {
            directory: utils::get_tests_directory(),
            filename: String::from("track02-coppied.flac"),
            ..Default::default()
        };

        match song::io::copy_song(&song, &mut copied_song) {
            Ok(_) => match copied_song.remove_from_filesystem() {
                Ok(_) => {}
                Err(err) => {
                    assert!(false, "Error: {err:?}")
                }
            },
            Err(err) => {
                assert!(false, "Error: {err:?}")
            }
        }
    }
}

#[cfg(test)]
mod album_tests {
    use crate::utils;
    use simodels::album;

    #[test]
    fn parse_album() {
        let test_dir = utils::get_tests_directory();
        if utils::does_directory_exists(&test_dir) {
            let album_file: String = test_dir + &String::from("album.json");
            println!("Album file: {:?}", album_file);

            match album::collection::parse_album(&album_file) {
                Ok(album) => {
                    println!("Album title: {}", album.title);
                    assert_eq!(album.title.is_empty(), false);
                    assert_eq!(album.artist.is_empty(), false);
                    assert_eq!(album.tracks.is_empty(), false);
                }
                Err(err) => {
                    assert!(false, "Error parsing album json file: {:?}", err);
                }
            }
        }
    }
}
