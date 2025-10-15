use crate::plex_client::deserializer::deserialize_integer_bool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaContainer {
    #[serde(rename = "@size")]
    pub size: u32,

    #[serde(rename = "Playlist")]
    pub playlists: Vec<Playlist>,
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    #[serde(rename = "@ratingKey")]
    pub rating_key: String,

    #[serde(rename = "@title")]
    pub title: String,

    #[serde(rename = "@key")]
    pub key: String,

    #[serde(rename = "@summary")]
    pub summary: String,

    #[serde(rename = "@smart", deserialize_with = "deserialize_integer_bool")]
    pub smart: bool,

    #[serde(rename = "@playlistType")]
    pub playlist_type: String,

    #[serde(rename = "@leafCount")]
    pub leaf_count: u32,

    #[serde(rename = "@addedAt")]
    pub added_at: u32,

    #[serde(rename = "@updatedAt")]
    pub updated_at: u32,
}

pub struct PlaylistFilter {
    pub only_playlist_type: Option<String>,
}

impl Playlist {
    pub fn to_string(&self) -> String {
        let mut output = format!(
            "{}: {} [{}]",
            self.rating_key, self.title, self.playlist_type
        );
        if self.smart {
            output = format!("{} [Smart]", output)
        }
        format!("{} [{} tracks]", output, self.leaf_count)
    }

    pub fn matches(&self, filter: &PlaylistFilter) -> bool {
        match &filter.only_playlist_type {
            None => true,
            Some(only_filter) => *only_filter == self.playlist_type,
        }
    }
}
