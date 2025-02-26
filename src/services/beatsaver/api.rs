use log::debug;

use super::map::BSMap;
use std::error::Error;

pub async fn get_map_data(id: &String) -> Result<BSMap, Box<dyn Error + Send + Sync>> {
    let res = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .text()
        .await?;

    debug!("{}", res);

    let map_data_res = serde_json::from_str::<BSMap>(&res);

    Ok(map_data_res?)
}
