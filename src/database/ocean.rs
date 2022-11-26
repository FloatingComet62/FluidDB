use super::low::{get_data, set_data};
use fluid_api::Error;
use json::{JsonValue, parse};

pub fn get_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            Ok(parse(data[ocean_name].to_string().as_str()).unwrap())
            // We can safely unwrap because we are sure it's a valid json
            // We also convert to string and then to &str to avoid the option type
        }
    }
}

pub fn create_ocean(ocean_name: &str) -> Result<(), Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            match data.insert(ocean_name, json::object! {}) {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(()) => {
                    match set_data(data) {
                        Err(e) => Err(e),
                        Ok(()) => Ok(()),
                    }
                }
            }

        }
    }
}

pub fn delete_ocean(ocean_name: &str) -> Result<(), Error> {
    match get_data() {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data[ocean_name].is_null() {
                return Err(Error::OceanNotExist);
            }
            match data.remove(ocean_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => {
                    match set_data(data) {
                        Err(e) => Err(e),
                        Ok(()) => Ok(()),
                    }
                }
            }

        }
    }
}
