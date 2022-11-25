use actix_web::{Responder, web::Path, get, post, delete, put};
use fluid_api::{res_handle_json, res_handle_null};
use super::super::database::river;

#[get("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn get_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::get_river(
        data.0.as_str(),
        data.1.as_str(),
        data.2.as_str()
    );
    res_handle_json(response)
}

#[post("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn post_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::create_river(
        data.0.as_str(),
        data.1.as_str(),
        data.2.as_str()
    );
    res_handle_null(response)
}

#[delete("/river/{ocean_name}/{sea_name}/{river_name}")]
pub async fn delete_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = river::delete_river(
        data.0.as_str(),
        data.1.as_str(),
        data.2.as_str()
    );
    res_handle_null(response)
}
#[put("/river/{ocean_name}/{sea_name}/{river_name}/{value}")]
pub async fn update_river(data: Path<(String, String, String, String)>) -> impl Responder {
    let response = river::update_river(
        data.0.as_str(),
        data.1.as_str(),
        data.2.as_str(),
        data.3.as_str()
    );
    res_handle_null(response)
}
