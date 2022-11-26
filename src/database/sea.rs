use super::low;
use fluid_api::Error;
use json::{JsonValue, parse};

pub fn get_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match low::get_data() {
        Err(e) => Err(e),
        Ok(data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            match parse(data[ocean_name][sea_name].to_string().as_str()) {
                Err(_) => Err(Error::FailedToParse),
                Ok(data) => Ok(data)
            }
            // We also convert to string and then to &str to avoid the option type
        }
    }
}

pub fn create_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match low::get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            match data[ocean_name].insert(sea_name, json::object! {}) {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(()) => {
                    match low::set_data(data) {
                        Err(e) => Err(e),
                        Ok(_e) => Ok(JsonValue::Null),
                    }
                }
            }
        }
    }
}

pub fn delete_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    match low::get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            match data[ocean_name].remove(sea_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => {
                    match low::set_data(data) {
                        Err(e) => Err(e),
                        Ok(_e) => Ok(JsonValue::Null),
                    }
                }
            }
        }
    }
}
