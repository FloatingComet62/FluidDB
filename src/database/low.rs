use std::fs::{read_to_string, write};
use super::Error;
use json::{parse, stringify, JsonValue};

pub fn get_data() -> Result<JsonValue, Error> {
    let data = read_to_string("data.json");
    match data {
        Err(_) => Err(Error::FailedToOpen),
        Ok(safe) => parser(&safe),
    }
}

fn parser(safe: &str) -> Result<JsonValue, Error> {
    match parse(safe) {
        Err(_) => Err(Error::FailedToParse),
        Ok(safeparsed) => Ok(safeparsed),
    }
}

pub fn set_data(obj: JsonValue) -> Result<JsonValue, Error> {
    let data = stringify(obj);
    let response = write("data.json", data.as_bytes());
    match response {
        Err(_) => Err(Error::FailedToSave),
        Ok(_) => Ok(JsonValue::Null),
    }
}
