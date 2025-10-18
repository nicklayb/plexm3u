use crate::m3u::{Item, Metadata, TrackData, WithMetadata};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MediaContainer {
    #[serde(rename = "Track", default)]
    pub tracks: Vec<Track>,

    #[serde(rename = "Video", default)]
    pub videos: Vec<Video>,

    #[serde(rename = "@ratingKey")]
    pub rating_key: u32,

    #[serde(rename = "@title")]
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct Video {
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@parentTitle")]
    pub parent_title: Option<String>,
    #[serde(rename = "@grandparentTitle")]
    pub grandparent_title: Option<String>,
    #[serde(rename = "Media")]
    pub medias: Vec<Media>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "@title")]
    pub title: String,
    #[serde(rename = "@parentTitle")]
    pub parent_title: Option<String>,
    #[serde(rename = "@grandparentTitle")]
    pub grandparent_title: Option<String>,
    #[serde(rename = "Media")]
    pub medias: Vec<Media>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Media {
    #[serde(rename = "Part")]
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Part {
    #[serde(rename = "@key")]
    pub key: String,
    #[serde(rename = "@file")]
    pub file: String,
}

impl WithMetadata for MediaContainer {
    fn metadata(&self) -> Vec<Metadata> {
        vec![
            Metadata::RatingKey(self.rating_key),
            Metadata::Title(self.title.clone()),
        ]
    }
}

pub trait WithMedia {
    fn full_title(&self) -> String;
    fn medias(&self) -> Vec<Media>;

    fn files(&self, rewrite_from: &Option<String>, rewrite_to: &Option<String>) -> Vec<Item> {
        let mut files: Vec<Item> = vec![];
        for media in self.medias().iter() {
            for part in media.parts.iter() {
                let mut file_name = part.file.clone();
                file_name = match rewrite_from {
                    Some(string) => {
                        let to = rewrite_to.clone().unwrap_or("".to_string());
                        file_name.replace(string, &to)
                    }
                    None => file_name,
                };
                let metadata = vec![TrackData::Key(part.key.clone())];
                let item = Item::new(file_name, metadata);
                files.push(item)
            }
        }

        files
    }
}

impl WithMedia for Track {
    fn medias(&self) -> Vec<Media> {
        self.medias.clone()
    }
    fn full_title(&self) -> String {
        format!(
            "{} - {} - {}",
            self.grandparent_title.clone().unwrap_or("".to_string()),
            self.parent_title.clone().unwrap_or("".to_string()),
            self.title
        )
    }
}

impl WithMedia for Video {
    fn medias(&self) -> Vec<Media> {
        self.medias.clone()
    }
    fn full_title(&self) -> String {
        format!(
            "{} - {} - {}",
            self.grandparent_title.clone().unwrap_or("".to_string()),
            self.parent_title.clone().unwrap_or("".to_string()),
            self.title
        )
    }
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
        for video in self.videos.iter() {
            for file in video.files(&rewrite_from, &rewrite_to).iter() {
                files.push(file.clone());
            }
        }

        files
    }
}
