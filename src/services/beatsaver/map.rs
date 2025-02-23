use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDetailMetadata {
    pub bpm: f64, 
    pub duration: i32,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapVersion {
    pub diffs: Vec<MapDetail>,
    #[serde(rename = "coverURL")]
    pub cover_url: String,
    #[serde(rename = "downloadURL")]
    pub download_url: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDetail {
    pub njs: f64,
    pub nps: f64,
    pub bombs: i32,
    pub obstacles: i32,
    pub bl_stars: Option<f64>,
    #[serde(rename = "stars")] 
    pub ss_stars: Option<f64>,
    pub events: i32,
    pub label: Option<String>,
    pub characteristic: String,
    pub difficulty: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub name: String,
    pub id: i32,
    pub avatar: String, // avatar URL
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapStats {
    pub upvotes: i32,
    pub downvotes: i32,
    pub score: f64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSMap {
    pub id: String,
    pub name: String, // song author - song name (sub name) 
    pub description: String, // with /r and /n and all that
    pub metadata: MapDetailMetadata,
    pub uploaded: DateTime<Utc>, 
    pub versions: Vec<MapVersion>, // https://discord.com/channels/882730837974609940/882731668589387796/1003157825553432616 
    pub uploader: UserDetail,
    pub collaborators: Option<Vec<UserDetail>>,
    pub stats: MapStats,
}