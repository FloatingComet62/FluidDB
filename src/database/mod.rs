use json::JsonValue;
use fluid_api::Error;

pub mod low;

pub mod ocean;
pub mod sea;
pub mod river;

pub fn get_everything() -> Result<JsonValue, Error> {
    low::get_data()
}
