use super::low::get_data;
use super::Error;
use super::super::{handle_set, null_check_ocean};
use json::{JsonValue, parse, object};

pub fn get_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    let data = get_data()?;
    let ocean = null_check_ocean(&data, &ocean_name)?;
    match parse(&ocean) {
        Err(_) => Err(Error::FailedToParse),
        Ok(data) => Ok(data)
    }
}

pub fn create_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    let mut data = get_data()?;
    match data.insert(ocean_name, object! {}) {
        Err(_) => Err(Error::FailedToUpdate),
        Ok(_) => handle_set(data)
    }
}

pub fn delete_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    let mut data = get_data()?;
    null_check_ocean(&data, &ocean_name)?;
    match data.remove(ocean_name) {
        JsonValue::Null => Err(Error::FailedToUpdate),
        _ => handle_set(data)
    }
}
