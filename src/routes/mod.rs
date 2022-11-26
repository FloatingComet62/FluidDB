use actix_web::{Responder, get};
use fluid_api::res_handler;
use super::database;

pub mod ocean;
pub mod sea;
pub mod river;

#[get("/")]
async fn home() -> impl Responder {
    let response = database::get_everything();
    res_handler(response)
}

#[get("/reset")]
async fn reset() -> impl Responder {
    let response = database::reset();
    res_handler(response)
}
