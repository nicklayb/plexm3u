use std::io::{self, prelude::*};
use std::path::PathBuf;
use std::{fs::File, path::Path};

const HEADER_LINE: &str = "#EXTM3U";

#[derive(Debug)]
pub struct M3U {
    pub tracks: Vec<Item>,
    pub metadata: Vec<Metadata>,
}

impl M3U {
    pub fn new(tracks: Vec<Item>, metadata: Vec<Metadata>) -> M3U {
        M3U { tracks, metadata }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub path: String,
    pub metadata: Vec<TrackData>,
}

#[derive(Debug, Clone)]
pub enum TrackData {
    Key(String),
}

#[derive(Debug)]
pub enum Metadata {
    RatingKey(u32),
    Title(String),
    RewriteFrom(String),
    RewriteTo(String),
    TrackData(TrackData),
}

pub trait WithMetadata {
    fn metadata(&self) -> Vec<Metadata>;
}

pub struct M3UAttribute {
    key: String,
    value: String,
}

impl M3UAttribute {
    pub fn new(key: String, value: String) -> M3UAttribute {
        M3UAttribute { key, value }
    }
    pub fn to_string(&self) -> String {
        format!("PLEXM3U_{}:{}", self.key, self.value)
    }
}

impl TrackData {
    fn format(&self) -> M3UAttribute {
        match self {
            TrackData::Key(key) => M3UAttribute::new("TRACK_KEY".to_string(), key.clone()),
        }
    }

    pub fn is_key(&self) -> bool {
        match self {
            TrackData::Key(_) => true,
        }
    }
}

impl Metadata {
    fn format(&self) -> M3UAttribute {
        match self {
            Metadata::RatingKey(rating_key) => {
                M3UAttribute::new("RATING_KEY".to_string(), rating_key.to_string())
            }
            Metadata::Title(title) => M3UAttribute::new("TITLE".to_string(), title.clone()),
            Metadata::RewriteFrom(rewrite_from) => {
                M3UAttribute::new("REWRITE_FROM".to_string(), rewrite_from.clone())
            }
            Metadata::RewriteTo(rewrite_to) => {
                M3UAttribute::new("REWRITE_TO".to_string(), rewrite_to.clone())
            }
            Metadata::TrackData(track_data) => track_data.format(),
        }
    }

    pub fn is_title(&self) -> bool {
        match self {
            Self::Title(_) => true,
            _ => false,
        }
    }

    pub fn parse(input: String) -> Option<Metadata> {
        if input.starts_with("#PLEXM3U_") {
            let cleaned = input.replace("#PLEXM3U_", "");
            let mut splitted = cleaned.splitn(2, ":");
            match (splitted.next(), splitted.next()) {
                (Some("RATING_KEY"), Some(rating_key)) => {
                    if let Ok(key_as_u32) = rating_key.parse::<u32>() {
                        Some(Metadata::RatingKey(key_as_u32))
                    } else {
                        None
                    }
                }
                (Some("TITLE"), Some(title)) => Some(Metadata::Title(title.to_string())),
                (Some("REWRITE_FROM"), Some(rewrite_from)) => {
                    Some(Metadata::RewriteFrom(rewrite_from.to_string()))
                }
                (Some("REWRITE_TO"), Some(rewrite_to)) => {
                    Some(Metadata::RewriteTo(rewrite_to.to_string()))
                }
                (Some("TRACK_KEY"), Some(track_key)) => {
                    Some(Metadata::TrackData(TrackData::Key(track_key.to_string())))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Item {
    pub fn new(path: String, metadata: Vec<TrackData>) -> Item {
        Item { path, metadata }
    }

    pub fn exists_at(&self, root_path: &Path) -> bool {
        let full_path = self.full_path(root_path);
        full_path.exists()
    }

    pub fn full_path(&self, root_path: &Path) -> PathBuf {
        root_path.join(self.path.clone())
    }

    pub fn track_key(&self) -> Option<String> {
        match self.metadata.iter().find(|track_data| track_data.is_key()) {
            Some(TrackData::Key(key)) => Some(key.clone()),
            _ => None,
        }
    }
}

pub fn write<P: AsRef<Path>>(filename: P, m3u: M3U) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "{}", HEADER_LINE)?;
    for meta in m3u.metadata.iter().clone() {
        writeln!(file, "#{}", meta.format().to_string())?;
    }
    if let Some(Metadata::Title(title)) = m3u.metadata.iter().find(|meta| meta.is_title()) {
        writeln!(file, "#PLAYLIST:{}", title)?;
    }
    for line in m3u.tracks {
        for meta in line.metadata {
            writeln!(file, "#{}", meta.format().to_string())?;
        }
        writeln!(file, "{}", line.path)?;
    }
    Ok(())
}

pub fn read<P: AsRef<Path>>(filename: P) -> std::io::Result<M3U> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let mut output_lines: Vec<Item> = Vec::new();
    let mut accumulated_track_meta: Vec<TrackData> = Vec::new();
    let mut output_metadata: Vec<Metadata> = Vec::new();

    if let Some(Ok(first_line)) = lines.next()
        && first_line == HEADER_LINE
    {
        for line_result in lines {
            let line_content = line_result?;
            match Metadata::parse(line_content.clone()) {
                Some(Metadata::TrackData(track_data)) => {
                    accumulated_track_meta.push(track_data);
                }
                Some(metadata) => output_metadata.push(metadata),
                None => {
                    if !line_content.starts_with("#") {
                        let item = Item::new(line_content, accumulated_track_meta);
                        output_lines.push(item);
                        accumulated_track_meta = Vec::new();
                    }
                }
            }
        }

        Ok(M3U {
            tracks: output_lines,
            metadata: output_metadata,
        })
    } else {
        panic!("M3u playlists must start with #EXTM3U")
    }
}
