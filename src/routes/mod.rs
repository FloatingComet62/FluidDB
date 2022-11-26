use actix_web::{Responder, get};
use fluid_api::res_handle_json;
use super::database;

pub mod ocean;
pub mod sea;
pub mod river;

#[get("/")]
async fn home() -> impl Responder {
    let response = database::get_everything();
    res_handle_json(response)
}
