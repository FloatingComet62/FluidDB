use actix_web::{Responder, web::Path, get, post, delete, put};
use super::res_handler;
use super::super::database::river;

#[get("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn get_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::get_river(&data.0, &data.1, &data.2);
    res_handler(response)
}

#[post("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn post_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::create_river(&data.0, &data.1, &data.2);
    res_handler(response)
}

#[delete("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn delete_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::delete_river(&data.0, &data.1, &data.2);
    res_handler(response)
}
#[put("/river/{ocean_name}/{sea_name}/{river_name}/{value}")]
pub async fn update_river(data: Path<(String, String, String, String)>) -> impl Responder {
    let response = river::update_river(&data.0, &data.1, &data.2, &data.3);
    res_handler(response)
}
