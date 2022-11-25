use actix_web::{get, Responder, HttpServer, App, web::Path, post, delete, put};
use fluid_api::{res_handle_json, res_handle_null};

mod database;

const PORT: u16 = 8080;

#[get("/")]
async fn home() -> impl Responder {
    let response = database::get_everything();
    res_handle_json(response)
}

#[get("/ocean/{ocean_name}")]
async fn get_ocean(data: Path<String>) -> impl Responder {
    let response = database::get_ocean(data.as_str());
    res_handle_json(response)
}

#[post("/ocean/{ocean_name}")]
async fn post_ocean(data: Path<String>) -> impl Responder {
    let response = database::create_ocean(data.as_str());
    res_handle_null(response)
}

#[delete("/ocean/{ocean_name}")]
async fn delete_ocean(data: Path<String>) -> impl Responder {
    let response = database::delete_ocean(data.as_str());
    res_handle_null(response)
}

#[get("/sea/{ocean_name}/{sea_name}")]
async fn get_sea(data: Path<(String, String)>) -> impl Responder {
    let response = database::get_sea(data.0.as_str(), data.1.as_str());
    res_handle_json(response)
}

#[post("/sea/{ocean_name}/{sea_name}")]
async fn post_sea(data: Path<(String, String)>) -> impl Responder {
    let response = database::create_sea(data.0.as_str(), data.1.as_str());
    res_handle_null(response)
}

#[delete("/sea/{ocean_name}/{sea_name}")]
async fn delete_sea(data: Path<(String, String)>) -> impl Responder {
    let response = database::delete_sea(data.0.as_str(), data.1.as_str());
    res_handle_null(response)
}

#[get("/river/{ocean_name}/{sea_name}/{river_name}")]
async fn get_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = database::get_river(data.0.as_str(), data.1.as_str(), data.2.as_str());
    res_handle_json(response)
}

#[post("/river/{ocean_name}/{sea_name}/{river_name}")]
async fn post_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = database::create_river(data.0.as_str(), data.1.as_str(), data.2.as_str());
    res_handle_null(response)
}

#[delete("/river/{ocean_name}/{sea_name}/{river_name}")]
async fn delete_river(data: Path<(String, String, String)>) -> impl Responder {
    let response = database::delete_river(data.0.as_str(), data.1.as_str(), data.2.as_str());
    res_handle_null(response)
}
#[put("/river/{ocean_name}/{sea_name}/{river_name}/{value}")]
async fn update_river(data: Path<(String, String, String, String)>) -> impl Responder {
    let response = database::update_river(data.0.as_str(), data.1.as_str(), data.2.as_str(), data.3.as_str());
    res_handle_null(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(get_ocean).service(post_ocean).service(delete_ocean)
            .service(get_sea).service(post_sea).service(delete_sea)
            .service(get_river).service(post_river).service(delete_river).service(update_river)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
