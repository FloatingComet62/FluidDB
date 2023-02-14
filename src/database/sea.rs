use super::low::get_data;
use super::Error;
use super::super::{handle_set, null_check_sea, null_check_ocean, match_hell_fix};
use json::{JsonValue, parse};

pub fn get_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(null_check_sea(&data, &ocean_name, &sea_name), {
            match parse(&sea) {
                Err(_) => Err(Error::FailedToParse),
                Ok(data) => Ok(data)
            }
        }, sea)
    }, data)
}

pub fn create_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(null_check_ocean(&data, &ocean_name), {
            match data[ocean_name].insert(sea_name, json::object! {}) {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(_) => handle_set(data)
            }
        }, _d)
    }, mut data)
}

pub fn delete_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(null_check_sea(&data, &ocean_name, &sea_name), {
            match data[ocean_name].remove(sea_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => handle_set(data)
            }
        }, _d)
    }, mut data)
}
