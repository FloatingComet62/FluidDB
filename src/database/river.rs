use super::low;
use fluid_api::Error;
use json::{JsonValue, parse};

pub fn get_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<JsonValue, Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(data) => {
            if data.has_key(ocean_name) {
                if data[ocean_name].has_key(sea_name) {
                    if data[ocean_name][sea_name].has_key(river_name) {
                        Ok(parse(data[ocean_name][sea_name][river_name].to_string().as_str()).unwrap())
                        // We can safely unwrap because we are sure it's a valid json
                        // We also convert to string and then to &str to avoid the option type
                    } else {
                        Err(Error::RiverNotExist)
                    }
                } else {
                    Err(Error::SeaNotExist)
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn create_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data.has_key(ocean_name) {
                if data[ocean_name].has_key(sea_name) {
                    match data[ocean_name][sea_name].insert(river_name, "") {
                        Err(_) => Err(Error::FailedToUpdate),
                        Ok(()) => {
                            match low::set_data(data) {
                                Err(e) => Err(e),
                                Ok(()) => Ok(()),
                            }
                        }
                    }
                } else {
                    Err(Error::SeaNotExist)
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn delete_river(ocean_name: &str, sea_name: &str, river_name: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data.has_key(ocean_name) {
                if data[ocean_name].has_key(sea_name) {
                    match data[ocean_name][sea_name].remove(river_name) {
                        JsonValue::Null => Err(Error::FailedToUpdate),
                        _ => {
                            match low::set_data(data) {
                                Err(e) => Err(e),
                                Ok(()) => Ok(()),
                            }
                        }
                    }
                } else {
                    Err(Error::SeaNotExist)
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}

pub fn update_river(ocean_name: &str, sea_name: &str, river_name: &str, value: &str) -> Result<(), Error> {
    let res = low::get_data();
    match res {
        Err(e) => Err(e),
        Ok(mut data) => {
            if data.has_key(ocean_name) {
                if data[ocean_name].has_key(sea_name) {
                    match data[ocean_name][sea_name].insert(river_name, value) {
                        Err(_) => Err(Error::FailedToUpdate),
                        Ok(()) => {
                            match low::set_data(data) {
                                Err(e) => Err(e),
                                Ok(()) => Ok(()),
                            }
                        }
                    }
                } else {
                    Err(Error::SeaNotExist)
                }
            } else {
                Err(Error::OceanNotExist)
            }
        }
    }
}
