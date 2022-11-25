use super::low;
use fluid_api::Error;
use json::{JsonValue, parse};

pub fn get_sea(ocean_name: &str, sea_name: &str) -> Result<JsonValue, Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(data) => {
            if data.has_key(ocean_name) {
                if data[ocean_name].has_key(sea_name) {
                    Ok(parse(data[ocean_name][sea_name].to_string().as_str()).unwrap())
                    // We can safely unwrap because we are sure it's a valid json
                    // We also convert to string and then to &str to avoid the option type
                } else {
                    Err(Error::SeaNotExist)
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn create_sea(ocean_name: &str, sea_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data.has_key(ocean_name) {
                match data[ocean_name].insert(sea_name, json::object! {}) {
                    Err(_) => Err(Error::FailedToUpdate),
                    Ok(()) => {
                        match low::set_data(data) {
                            Err(e) => Err(e),
                            Ok(()) => Ok(()),
                        }
                    }
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn delete_sea(ocean_name: &str, sea_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data.has_key(ocean_name) {
                match data[ocean_name].remove(sea_name) {
                    JsonValue::Null => Err(Error::FailedToUpdate),
                    _ => {
                        match low::set_data(data) {
                            Err(e) => Err(e),
                            Ok(()) => Ok(()),
                        }
                    }
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}
