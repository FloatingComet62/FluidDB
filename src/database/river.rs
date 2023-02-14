use super::low::get_data;
use super::Error;
use super::super::{handle_set, null_check_river, null_check_sea, match_hell_fix};
use json::JsonValue;

pub fn get_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(
            null_check_river(&data, &ocean_name, &sea_name, &river_name),
            { Ok(JsonValue::String(river)) },
            river
        )
    }, data)
}

pub fn create_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(
            null_check_sea(&data, &ocean_name, &sea_name), {
                match data[ocean_name][sea_name].insert(river_name, "") {
                    Err(_) => Err(Error::FailedToUpdate),
                    Ok(_) => handle_set(data)
                }
            },
            _d
        )
    }, mut data)
}

pub fn delete_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match data[ocean_name][sea_name].remove(river_name) {
            JsonValue::Null => Err(Error::FailedToUpdate),
            _ => handle_set(data)
        }
    }, mut data)
}

pub fn update_river(ocean_name: &str, sea_name: &str, river_name: &str, value: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(
            null_check_river(&data, &ocean_name, &sea_name, &river_name),
            {
                match data[ocean_name][sea_name].insert(river_name, value) {
                    Err(_) => Err(Error::FailedToUpdate),
                    Ok(_) => handle_set(data)
                }
            },
            _d
        )
    }, mut data)
}
