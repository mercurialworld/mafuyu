use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDetailMetadata {
    bpm: f64, 
    duration: i32,
    song_name: String,
    song_sub_name: String,
    song_author_name: String,
    level_author_name: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapVersion {
    diffs: Vec<MapDetail>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapDetail {
    njs: f64,
    nps: f64,
    bombs: i32,
    obstacles: i32,
    bl_stars: Option<f64>,
    #[serde(rename = "stars")] 
    ss_stars: Option<f64>,
    events: i32,
    label: Option<String>,
    characteristic: String,
    difficulty: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    name: String,
    id: i32,
    avatar: String, // avatar URL
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSMap {
    id: String,
    name: String, // song author - song name (sub name) 
    description: String, // with /r and /n and all that
    metadata: MapDetailMetadata,
    uploaded: DateTime, 
    versions: Vec<MapVersion>, // https://discord.com/channels/882730837974609940/882731668589387796/1003157825553432616 
    uploader: UserDetail,
    collaborators: Option<Vec<UserDetail>>
}