use beatsaver_api::client::BeatSaverClient;
use regex::Regex;

pub mod commands;
pub mod core;
pub mod ui;
pub mod utils;

pub struct Data {
    pub beatsaver_client: BeatSaverClient,
    pub bsr_link_regex: Regex,
    pub hexstring_regex: Regex,
} // User data, which is stored and accessible in all command invocations
pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;
