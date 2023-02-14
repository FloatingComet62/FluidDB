use actix_web::{
    HttpResponse,
    Responder,
    http::{
        StatusCode,
        header::ContentType,
    }, web::Json,
};
use json::{JsonValue, stringify};

pub mod database;
pub mod routes;

pub enum Error {
    FailedToParse,
    FailedToSave,
    FailedToOpen,
    FailedToUpdate,

    OceanNotExist,
    SeaNotExist,
    RiverNotExist,
}

struct ErrorData {
    code: StatusCode,
    msg: String,
}

fn error_data(e: Error) -> ErrorData {
    let mut ise = ErrorData { code: StatusCode::INTERNAL_SERVER_ERROR, msg: String::new() };
    let mut br = ErrorData { code: StatusCode::BAD_REQUEST, msg: String::new() };
    match e {
        Error::FailedToParse => { ise.msg = String::from("Failed to parse");ise },
        Error::FailedToSave => { ise.msg = String::from("Failed to save");ise },
        Error::FailedToOpen => { ise.msg = String::from("Failed to open");ise },
        Error::FailedToUpdate => { ise.msg = String::from("Failed to update");ise },

        Error::OceanNotExist => { br.msg = String::from("Ocean doesn't exist");br },
        Error::SeaNotExist =>  { br.msg = String::from("Sea doesn't exist");br },
        Error::RiverNotExist => { br.msg = String::from("River doesn't exist");br },
    }
}

pub fn res_handler(response: Result<JsonValue, Error>) -> impl Responder {
    match response {
        Err(e) => {
            let data = error_data(e);
            HttpResponse::build(data.code)
                .insert_header(ContentType::plaintext())
                .body(data.msg)
        }
        Ok(data) => HttpResponse::Ok().body(stringify(data))
    }
}

pub fn handle_set(data: JsonValue) -> Result<JsonValue, Error> {
    match database::low::set_data(data) {
        Err(e) => Err(e),
        Ok(_) => Ok(JsonValue::Null),
    }
}

pub fn null_check_river(data: &JsonValue, ocean_name: &str, sea_name: &str, river_name: &str) -> Result<String, Error> {
    if data[ocean_name].is_null() {
        return Err(Error::OceanNotExist);
    }
    if data[ocean_name][sea_name].is_null() {
        return Err(Error::SeaNotExist);
    }
    if data[ocean_name][sea_name][river_name].is_null() {
        return Err(Error::RiverNotExist);
    }
    return Ok(data[ocean_name][sea_name][river_name].to_string());
}
pub fn null_check_sea(data: &JsonValue, ocean_name: &str, sea_name: &str) -> Result<String, Error> {
    if data[ocean_name].is_null() {
        return Err(Error::OceanNotExist);
    }
    if data[ocean_name][sea_name].is_null() {
        return Err(Error::SeaNotExist);
    }
    return Ok(data[ocean_name][sea_name].to_string());
}
pub fn null_check_ocean(data: &JsonValue, ocean_name: &str) -> Result<String, Error> {
    if data[ocean_name].is_null() {
        return Err(Error::OceanNotExist);
    }
    return Ok(data[ocean_name].to_string());
}

#[macro_export]
macro_rules! match_hell_fix {
    ($to_match: expr, $ok_statement: block, $( $var: ident ) *) => {
        match $to_match {
            Err(e) => Err(e),
            Ok($($var )*) => {
                $ok_statement
            }
        }
    };
}