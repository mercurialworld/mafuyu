use crate::Error;
use super::map::BSMap;

pub async fn get_map_data(id: String) -> Result<BSMap, Error> {
    let res = reqwest::get(format!("https://api.beatsaver.com/maps/id/{}", id))
        .await?
        .text()
        .await?;

    let map_data_res = serde_json::from_str::<BSMap>(&res);
    
    let map_data = match map_data_res {
        Ok(data) => data,
        Err(err) => err,
    };

    Ok(map_data)
}