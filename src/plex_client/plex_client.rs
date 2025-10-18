use quick_xml::de::from_str;
use reqwest::blocking::Response;

use crate::plex_client::playlist::MediaContainer as PlaylistMediaContainer;
use crate::plex_client::track::MediaContainer as TrackMediaContainer;
use log::info;

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
        let body = self.get_as_text("/playlists".to_string());

        match from_str::<PlaylistMediaContainer>(&body) {
            Ok(media_container) => media_container,
            Err(error) => panic!("{:#?}", error),
        }
    }

    pub fn get_playlist(&self, rating_key: String) -> TrackMediaContainer {
        let body = self.get_as_text(format!("/playlists/{}/items", rating_key).to_string());

        info!("{:?}", body);

        match from_str::<TrackMediaContainer>(&body) {
            Ok(media_container) => media_container,
            Err(error) => panic!("{:#?}", error),
        }
    }

    pub fn get_part(&self, part_key: String) -> Response {
        self.get_response(part_key)
    }

    fn get_response(&self, path: String) -> Response {
        self.call(path)
    }

    fn get_as_text(&self, path: String) -> String {
        let response = self.call(path);
        match response.text() {
            Ok(body) => body,
            Err(error) => panic!("{:#?}", error),
        }
    }

    fn call(&self, path: String) -> Response {
        let client_url = self.client_url(path);
        info!("GET {}", client_url);
        match reqwest::blocking::get(client_url) {
            Ok(response) => response,
            Err(error) => panic!("{:#?}", error),
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
