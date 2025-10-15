use quick_xml::de::from_str;
use serde::Deserialize;

use log::{debug, error, info};

pub struct PlexClient {
    pub server: String,
    token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MediaContainer {
    #[serde(rename = "@size")]
    pub size: u32,

    #[serde(rename = "Playlist")]
    pub playlists: Vec<Playlist>,
}

// <Playlist
// ratingKey="135542"
// key="/playlists/135542/items"
// guid="com.plexapp.agents.none://f2e8a9e5-dc6d-4fcc-8429-7e1c028309f3"
// type="playlist"
// title="Best Albums"
// summary=""
// smart="1"
// playlistType="audio"
// composite="/playlists/135542/composite/1740942826"
// icon="playlist://image.smart"
// viewCount="30"
// lastViewedAt="1757346353"
// duration="376003000" leafCount="1589" addedAt="1735592208" updatedAt="1740942826"> </Playlist>
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

    pub fn matches(&self, only: &Option<String>) -> bool {
        match only {
            None => true,
            Some(only_filter) => *only_filter == self.playlist_type,
        }
    }
}

fn deserialize_integer_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(string) => Ok(string == "1"),
        Err(_) => Ok(false),
    }
}

fn parse_playlists(xml: &str) -> Result<MediaContainer, quick_xml::DeError> {
    from_str::<MediaContainer>(xml)
}

impl PlexClient {
    pub fn new(server: String, token: Option<String>) -> PlexClient {
        PlexClient {
            server: server,
            token: token,
        }
    }
    pub fn list_playlists(&self) -> MediaContainer {
        let body = self.call("/playlists".to_string());

        match parse_playlists(&body) {
            Ok(media_container) => media_container,
            Err(error) => panic!("{:#?}", error),
        }
    }

    pub fn get_playlist(&self, rating_key: String) {
        let body = self.call(format!("/playlists/{}/items", rating_key).to_string());
        println!("{:?}", body);

        // match parse_playlists(&body) {
        //     Ok(media_container) => media_container,
        //     Err(error) => panic!("{:#?}", error),
        // }
    }

    fn call(&self, path: String) -> String {
        let client_url = self.client_url(path);
        info!("GET {}", client_url);
        let result = reqwest::blocking::get(client_url);
        match result {
            Ok(response) => match response.text() {
                Ok(body) => body,
                Err(error) => panic!("{:#?}", error),
            },
            Err(ref response) => panic!("{:#?}", response),
        }
    }

    fn client_url(&self, path: String) -> String {
        format!("{}{}", self.server, path)
    }
}
