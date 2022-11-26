use actix_web::{
    HttpResponse,
    Responder,
    http::{
        StatusCode,
        header::ContentType,
    },
};
use json::{JsonValue, stringify};

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
        Error::FailedToParse => { ise.msg = String::from("Failed to parse");br },
        Error::FailedToSave => { ise.msg = String::from("Failed to save");br },
        Error::FailedToOpen => { ise.msg = String::from("Failed to open");br },
        Error::FailedToUpdate => { ise.msg = String::from("Failed to update");br },

        Error::OceanNotExist => { br.msg = String::from("Ocean doesn't exist");br },
        Error::SeaNotExist =>  { br.msg = String::from("Sea doesn't exist");br },
        Error::RiverNotExist => { br.msg = String::from("River doesn't exist");br },
    }
}

pub fn res_handle_json(response: Result<JsonValue, Error>) -> impl Responder {
    match response {
        Err(e) => {
            let data = error_data(e);
            HttpResponse::build(data.code)
                .insert_header(ContentType::plaintext())
                .body(data.msg)
        }
        Ok(data) => {
            HttpResponse::Ok().body(stringify(data))
        }
    }
}
pub fn res_handle_null(response: Result<(), Error>) -> impl Responder {
    match response {
        Err(e) => {
            let data = error_data(e);
            HttpResponse::build(data.code)
                .insert_header(ContentType::plaintext())
                .body(data.msg)
        }
        Ok(_) => {
            HttpResponse::Ok().body("")
        }
    }
}
