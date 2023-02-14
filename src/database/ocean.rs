use super::low::get_data;
use super::Error;
use super::super::{handle_set, null_check_ocean, match_hell_fix};
use json::{JsonValue, parse, object};

pub fn get_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(null_check_ocean(&data, &ocean_name), {
            match parse(&ocean) {
                Err(_) => Err(Error::FailedToParse),
                Ok(data) => Ok(data)
            }
        }, ocean)
    }, data)
}

pub fn create_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match data.insert(ocean_name, object! {}) {
            Err(_) => Err(Error::FailedToUpdate),
            Ok(_) => handle_set(data)
        }
    }, mut data)
}

pub fn delete_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    match_hell_fix!(get_data(), {
        match_hell_fix!(null_check_ocean(&data, &ocean_name), {
            match data.remove(ocean_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => handle_set(data)
            }
        }, _d)
    }, mut data)
}
