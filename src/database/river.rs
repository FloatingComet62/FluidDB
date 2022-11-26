use super::low::{get_data, set_data};
use fluid_api::Error;
use json::JsonValue;

pub fn get_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            if data[ocean_name][sea_name][river_name].is_null() {
                return Err(Error::RiverNotExist);
            }

            Ok(JsonValue::String(data[ocean_name][sea_name][river_name].to_string()))
        }
    }
}

pub fn create_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            match data[ocean_name][sea_name].insert(river_name, "") {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(()) => {
                    match set_data(data) {
                        Err(e) => Err(e),
                        Ok(_e) => Ok(JsonValue::Null),
                    }
                }
            }
        }
    }
}

pub fn delete_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            if data[ocean_name][sea_name][river_name].is_null() {
                return Err(Error::RiverNotExist);
            }
            match data[ocean_name][sea_name].remove(river_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => {
                    match set_data(data) {
                        Err(e) => Err(e),
                        Ok(_e) => Ok(JsonValue::Null),
                    }
                }
            }
        }
    }
}

pub fn update_river(ocean_name: &str, sea_name: &str, river_name: &str, value: &str) -> Result<JsonValue, Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            if data[ocean_name][sea_name].is_null() {
                return Err(Error::SeaNotExist);
            }
            if data[ocean_name][sea_name][river_name].is_null() {
                return Err(Error::RiverNotExist);
            }
            match data[ocean_name][sea_name].insert(river_name, value) {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(()) => {
                    match set_data(data) {
                        Err(e) => Err(e),
                        Ok(_e) => Ok(JsonValue::Null),
                    }
                }
            }
        }
    }
}
