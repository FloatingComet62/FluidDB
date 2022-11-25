use super::low;
use fluid_api::Error;
use json::{JsonValue, parse};

pub fn get_ocean(ocean_name: &str) -> Result<JsonValue, Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(data) => {
            if data.has_key(ocean_name) {
                Ok(parse(data[ocean_name].to_string().as_str()).unwrap())
                // We can safely unwrap because we are sure it's a valid json
                // We also convert to string and then to &str to avoid the option type
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn create_ocean(ocean_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            match data.insert(ocean_name, json::object! {}) {
                Err(_) => Err(Error::FailedToUpdate),
                Ok(()) => {
                    match low::set_data(data) {
                        Err(e) => Err(e),
                        Ok(()) => Ok(()),
                    }
                }
            }

        }
    }
}

pub fn delete_ocean(ocean_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            match data.remove(ocean_name) {
                JsonValue::Null => Err(Error::FailedToUpdate),
                _ => {
                    match low::set_data(data) {
                        Err(e) => Err(e),
                        Ok(()) => Ok(()),
                    }
                }
            }

        }
    }
}
