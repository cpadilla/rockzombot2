use reqwest::Client;
use serde::{Deserialize, Serialize};
use http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artists {
    pub items: Vec<Artist>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub album: Album,
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disallows {
    pub resuming: Option<bool>,
    pub toggling_repeat_context: Option<bool>,
    pub toggling_repeat_track: Option<bool>,
    pub toggling_shuffle: Option<bool>,
    pub skipping_prev: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actions {
    pub disallows: Disallows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub timestamp: Option<u64>,
    pub context: Option<Context>,
    pub progress_ms: Option<u64>,
    pub item: Option<Item>,
    pub currently_playing_type: Option<String>,
    pub actions: Option<Actions>,
    pub is_playing: Option<bool>,
}

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    scope: String,
}

async fn refresh_token(client_id: &str, client_secret: &str, refresh_token: &str) -> TokenResponse {
    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .unwrap();
    response.json::<TokenResponse>().await.unwrap()
}

pub async fn get_current_song() -> Option<Song> {
    let client = reqwest::Client::new();
    let spotify_token = std::env::var("SPOTIFY_TOKEN").ok().unwrap();
    let auth = format!("{}{}", "Bearer ", spotify_token);
    let response = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header(AUTHORIZATION, auth)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<Song>().await {
                Ok(parsed) => {
                    println!("Success! {:?}", parsed);
                    Some(parsed)
                }
                Err(_) => {
                    println!("Hm, the response didn't match the shape we expected.");
                    None::<Song>
                }
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");

            let client_id = std::env::var("SPOTIFY_CLIENT_ID").ok().unwrap();
            let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET").ok().unwrap();
            let refresh_tok = std::env::var("SPOTIFY_REFRESH_TOKEN").ok().unwrap();
            let token_response = refresh_token(client_id.as_str(), client_secret.as_str(), refresh_tok.as_str()).await;
            let auth = format!("{}{}", "Bearer ", token_response.access_token);
            let response = client
                .get("https://api.spotify.com/v1/me/player/currently-playing")
                .header(AUTHORIZATION, auth)
                .header(CONTENT_TYPE, "application/json")
                .header(ACCEPT, "application/json")
                .send()
                .await
                .unwrap();

            match response.status() {
                reqwest::StatusCode::OK => {
                    // on success, parse our JSON to an APIResponse
                    let song = match response.json::<Song>().await {
                        Ok(parsed) => {
                            println!("Success! {:?}", parsed);
                            Some(parsed)
                        }
                        Err(_) => {
                            println!("Hm, the response didn't match the shape we expected.");
                            None::<Song>
                        }
                    };
                    song
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    //Uh oh! Failed to authorize!
                    None::<Song>
                }
                _ => {
                    //Uh oh! Something unexpected happened
                    None::<Song>
                }
            }
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

