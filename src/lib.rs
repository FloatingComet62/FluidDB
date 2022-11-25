pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub enum Error {
    FailedToParse,
    FailedToSave,
    FailedToOpen,
    FailedToUpdate,
    OceanNotExist,
    SeaNotExist,
    RiverNotExist,
}

use actix_web::{HttpResponse, Responder};
use json::JsonValue;
use json::stringify;

pub fn res_handle_json(response: Result<JsonValue, Error>) -> impl Responder {
    match response {
        Err(Error::FailedToSave) => HttpResponse::Ok().body("Failed to save the database"),
        Err(Error::FailedToOpen) => HttpResponse::Ok().body("Failed to open the database"),
        Err(Error::FailedToParse) => HttpResponse::Ok().body("Failed to parse the database"),
        Err(Error::FailedToUpdate) => HttpResponse::Ok().body("Failed to update the database"),
        Err(Error::OceanNotExist) => HttpResponse::Ok().body("Ocean doesn't exist"),
        Err(Error::SeaNotExist) => HttpResponse::Ok().body("Sea doesn't exist"),
        Err(Error::RiverNotExist) => HttpResponse::Ok().body("River doesn't exist"),
        Ok(data) => {
            HttpResponse::Ok().body(stringify(data))
        }
    }
}
pub fn res_handle_null(response: Result<(), Error>) -> impl Responder {
    match response {
        Err(Error::FailedToSave) => HttpResponse::Ok().body("Failed to save the database"),
        Err(Error::FailedToOpen) => HttpResponse::Ok().body("Failed to open the database"),
        Err(Error::FailedToParse) => HttpResponse::Ok().body("Failed to parse the database"),
        Err(Error::FailedToUpdate) => HttpResponse::Ok().body("Failed to update the database"),
        Err(Error::OceanNotExist) => HttpResponse::Ok().body("Ocean doesn't exist"),
        Err(Error::SeaNotExist) => HttpResponse::Ok().body("Sea doesn't exist"),
        Err(Error::RiverNotExist) => HttpResponse::Ok().body("River doesn't exist"),
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
    }
}
