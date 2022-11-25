use actix_web::{Responder, web::Path, get, post, delete};
use fluid_api::{res_handle_json, res_handle_null};
use super::super::database::sea;

#[get("/sea/{ocean_name}/{sea_name}")]
pub async fn get_sea(data: Path<(String, String)>) -> impl Responder {
    let response = sea::get_sea(
        data.0.as_str(),
        data.1.as_str()
    );
    res_handle_json(response)
}

#[post("/sea/{ocean_name}/{sea_name}")]
pub async fn post_sea(data: Path<(String, String)>) -> impl Responder {
    let response = sea::create_sea(
        data.0.as_str(),
        data.1.as_str()
    );
    res_handle_null(response)
}

#[delete("/sea/{ocean_name}/{sea_name}")]
pub async fn delete_sea(data: Path<(String, String)>) -> impl Responder {
    let response = sea::delete_sea(
        data.0.as_str(),
        data.1.as_str()
    );
    res_handle_null(response)
}
