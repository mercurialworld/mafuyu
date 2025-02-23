use std::error::Error;
use super::map::BSMap;

pub async fn get_map_data(id: &String) -> Result<BSMap, Box<dyn Error>> {
    let res = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .text()
        .await?;

    let map_data_res = serde_json::from_str::<BSMap>(&res);
    
    Ok(map_data_res?)
}