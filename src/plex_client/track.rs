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
    #[serde(rename = "@ratingKey")]
    pub rating_key: u32,
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
    #[serde(rename = "@ratingKey")]
    pub rating_key: u32,
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
    #[serde(rename = "@id")]
    pub id: u32,
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

pub enum TrackPath {
    File,
    Key(String),
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
    fn key(&self) -> String;
    fn informations(&self) -> Vec<(&str, Option<String>)>;

    fn medias(&self) -> Vec<Media>;

    fn extra_metadata(&self) -> Vec<TrackData>;

    fn print_informations(&self) {
        println!("\nKey: {}", self.key());
        for (title, value) in self.informations() {
            if let Some(inner_value) = value {
                println!("{}: {}", title, inner_value)
            }
        }
        println!("Medias:");
        for media in self.medias() {
            println!("- ID: {}", media.id);
            for part in media.parts {
                println!("  {} ({})", part.file, part.key);
            }
        }
    }

    fn files(
        &self,
        rewrite_from: &Option<String>,
        rewrite_to: &Option<String>,
        track_path: &TrackPath,
    ) -> Vec<Item> {
        let mut files: Vec<Item> = vec![];
        for media in self.medias().iter() {
            for part in media.parts.iter() {
                let mut file_name = match track_path {
                    TrackPath::File => part.file.clone(),
                    TrackPath::Key(base_host) => format!("{}{}", base_host, part.key.clone()),
                };
                file_name = match rewrite_from {
                    Some(string) => {
                        let to = rewrite_to.clone().unwrap_or("".to_string());
                        file_name.replace(string, &to)
                    }
                    None => file_name,
                };
                let mut metadata = vec![
                    TrackData::Key(part.key.clone()),
                    TrackData::FilePath(part.file.clone()),
                ];
                let mut extra_metadata = self.extra_metadata();
                metadata.append(&mut extra_metadata);
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

    fn key(&self) -> String {
        self.rating_key.to_string()
    }

    fn extra_metadata(&self) -> Vec<TrackData> {
        let mut metadata = vec![];
        if let Some(artist) = self.grandparent_title.clone() {
            metadata.push(TrackData::ExtM3u("EXTART".to_string(), artist.clone()))
        }
        if let Some(album) = self.parent_title.clone() {
            metadata.push(TrackData::ExtM3u("EXTALB".to_string(), album.clone()))
        }
        metadata
    }

    fn informations(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("Title", Some(self.title.clone())),
            ("Artist", self.grandparent_title.clone()),
            ("Album", self.parent_title.clone()),
        ]
    }
}

impl WithMedia for Video {
    fn medias(&self) -> Vec<Media> {
        self.medias.clone()
    }

    fn key(&self) -> String {
        self.rating_key.to_string()
    }

    fn informations(&self) -> Vec<(&str, Option<String>)> {
        vec![
            ("Title", Some(self.title.clone())),
            ("Show", self.grandparent_title.clone()),
            ("Season", self.parent_title.clone()),
        ]
    }

    fn extra_metadata(&self) -> Vec<TrackData> {
        let mut metadata = vec![];
        if let Some(artist) = self.grandparent_title.clone() {
            metadata.push(TrackData::ExtM3u("EXTART".to_string(), artist.clone()))
        }
        if let Some(album) = self.parent_title.clone() {
            metadata.push(TrackData::ExtM3u("EXTALB".to_string(), album.clone()))
        }
        metadata
    }
}

impl MediaContainer {
    pub fn track_files(
        &self,
        rewrite_from: Option<String>,
        rewrite_to: Option<String>,
        track_path: TrackPath,
    ) -> Vec<Item> {
        let mut files: Vec<Item> = vec![];
        for track in self.tracks.iter() {
            for file in track.files(&rewrite_from, &rewrite_to, &track_path).iter() {
                files.push(file.clone());
            }
        }
        for video in self.videos.iter() {
            for file in video.files(&rewrite_from, &rewrite_to, &track_path).iter() {
                files.push(file.clone());
            }
        }

        files
    }
}
