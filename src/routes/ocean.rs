use actix_web::{Responder, web::Path, get, post, delete};
use fluid_api::res_handler;
use super::super::database::ocean;

#[get("/ocean/{ocean_name}")]
pub async fn get_ocean(data: Path<String>) -> impl Responder {
    let response = ocean::get_ocean(data.as_str());
    res_handler(response)
}

#[post("/ocean/{ocean_name}")]
pub async fn post_ocean(data: Path<String>) -> impl Responder {
    let response = ocean::create_ocean(data.as_str());
    res_handler(response)
}

#[delete("/ocean/{ocean_name}")]
pub async fn delete_ocean(data: Path<String>) -> impl Responder {
    let response = ocean::delete_ocean(data.as_str());
    res_handler(response)
}
