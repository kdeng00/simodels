pub mod collection {
    use serde::{Deserialize, Serialize};
    use std::default::Default;

    use std::fs::File;
    use std::io::BufReader;

    use crate::init;

    pub fn parse_album(filepath: &String) -> Result<Album, serde_json::Error> {
        let file = File::open(filepath).expect("Failed to open file");
        let reader = BufReader::new(file);

        serde_json::from_reader(reader)
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Album {
        #[serde(skip_serializing_if = "String::is_empty")]
        #[serde(alias = "album")]
        pub title: String,
        #[serde(skip_serializing_if = "String::is_empty")]
        #[serde(alias = "album_artist")]
        pub artist: String,
        pub genre: String,
        pub year: i32,
        pub track_count: i32,
        #[serde(skip_serializing_if = "init::is_set")]
        pub disc_count: i32,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        pub tracks: Vec<Track>,
    }

    #[derive(Clone, Debug, Default, Deserialize, Serialize)]
    pub struct Track {
        pub title: String,
        pub artist: String,
        pub disc: i32,
        pub track: i32,
        /// In seconds
        pub duration: f64,
    }

    pub fn deserialize_json(json_data: &str, album: &mut Album) -> Result<(), serde_json::Error> {
        match serde_json::from_str::<Album>(json_data) {
            Ok(parsed_album) => {
                *album = parsed_album;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserilize_album_collection_json() {
        let album_file_path = "tests/album.json";
        let album_file_content = match std::fs::read_to_string(album_file_path) {
            Ok(content) => content,
            Err(err) => {
                assert!(false, "Error: {err:?}");
                String::new()
            }
        };

        let mut album = super::collection::Album::default();
        match super::collection::deserialize_json(&album_file_content, &mut album) {
            Ok(_) => {
                assert_eq!(3, album.tracks.len(), "Track count do not match");
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        }
    }
}
