use json::{JsonValue, object};
use fluid_api::Error;

pub mod low;

pub mod ocean;
pub mod sea;
pub mod river;

pub fn get_everything() -> Result<JsonValue, Error> {
    low::get_data()
}

pub fn reset() -> Result<JsonValue, Error> {
    low::set_data(object! {})
}
