use quick_xml::de::from_str;
use serde::Deserialize;

use crate::plex_client::playlist::MediaContainer as PlaylistMediaContainer;
use crate::plex_client::track::MediaContainer as TrackMediaContainer;

use log::{debug, error, info};

pub struct PlexClient {
    pub server: String,
    token: Option<String>,
}

impl PlexClient {
    pub fn new(server: String, token: Option<String>) -> PlexClient {
        PlexClient {
            server: server,
            token: token,
        }
    }
    pub fn list_playlists(&self) -> PlaylistMediaContainer {
        let body = self.call("/playlists".to_string());

        match from_str::<PlaylistMediaContainer>(&body) {
            Ok(media_container) => media_container,
            Err(error) => panic!("{:#?}", error),
        }
    }

    pub fn get_playlist(&self, rating_key: String) -> TrackMediaContainer {
        let body = self.call(format!("/playlists/{}/items", rating_key).to_string());

        match from_str::<TrackMediaContainer>(&body) {
            Ok(media_container) => media_container,
            Err(error) => panic!("{:#?}", error),
        }
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
        let mut url = format!("{}{}", self.server, path);
        if let Some(token) = &self.token {
            url = format!("{}?X-Plex-Token={}", url, token);
        }
        url
    }
}
