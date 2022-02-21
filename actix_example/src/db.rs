use std::fmt::Debug;
use std::fs::File;
use std::option::Option;


/// Define the database struct
/// It contains a list of songs
#[derive(Serialize, Deserialize)]
pub struct Database {
    songs: Vec<Song>
}

impl Database {
    /// Create a method which returns a specific song given an index
    pub fn get_song(&self, index: usize) -> Option<Song> {
        if index >= self.songs.len() {
            return None;
        }

        Some(self.songs.get(index).unwrap().clone())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Song {
    name: String,
    author: String,
    duration: u16
}

impl Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database").field("songs", &self.songs).finish()
    }
}

impl Debug for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Song").field("name", &self.name).field("author", &self.author).field("duration", &self.duration).finish()
    }
}

/// Read the JSON file from disk and parse it into a `Database` struct
pub fn parse_db() -> Database {
    let filename = "src/data.json";
    let file = File::open(filename)
        .expect("Unable to open file");

    serde_json::from_reader(file).expect("error while reading json")
}