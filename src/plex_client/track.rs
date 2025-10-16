use crate::{m3u::Item, plex_client::deserializer::deserialize_integer_bool};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaContainer {
    #[serde(rename = "@size")]
    pub size: u32,

    #[serde(rename = "Track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "Media")]
    pub medias: Vec<Media>,
    #[serde(default, rename = "Genre")]
    pub genres: Vec<Genre>,
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@parentTitle")]
    pub parent_title: Option<String>,
    #[serde(rename = "@grandparentTitle")]
    pub grandparent_title: Option<String>,
    #[serde(rename = "@parentRatingKey")]
    pub parent_rating_key: Option<u32>,
    #[serde(rename = "@grandparentRatingKey")]
    pub grandparent_rating_key: Option<u32>,
    #[serde(rename = "@index")]
    pub index: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Media {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@duration")]
    pub duration: u32,
    #[serde(rename = "Part")]
    pub parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
pub struct Part {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@size")]
    pub size: u32,
    #[serde(rename = "@container")]
    pub container: String,
    #[serde(rename = "@file")]
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    #[serde(rename = "@tag")]
    pub tag: String,
}

impl MediaContainer {
    pub fn track_files(
        &self,
        rewrite_from: Option<String>,
        rewrite_to: Option<String>,
    ) -> Vec<Item> {
        let mut files: Vec<Item> = vec![];
        for track in self.tracks.iter() {
            for file in track.files(&rewrite_from, &rewrite_to).iter() {
                files.push(file.clone());
            }
        }

        files
    }
}

impl Track {
    pub fn files(&self, rewrite_from: &Option<String>, rewrite_to: &Option<String>) -> Vec<Item> {
        let mut files: Vec<Item> = vec![];
        for media in self.medias.iter() {
            for part in media.parts.iter() {
                let mut file_name = part.file.clone();
                file_name = match rewrite_from {
                    Some(string) => {
                        let to = rewrite_to.clone().unwrap_or("".to_string());
                        file_name.replace(string, &to)
                    }
                    None => file_name,
                };
                let item = Item::new(file_name);
                files.push(item)
            }
        }

        files
    }
}
