use actix_web::{Responder, HttpServer, App, get};
use fluid_api::res_handle_json;

mod database;
mod routes;

const PORT: u16 = 8080;

#[get("/")]
async fn home() -> impl Responder {
    let response = database::get_everything();
    res_handle_json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)

            .service(routes::ocean::get_ocean)
            .service(routes::ocean::post_ocean)
            .service(routes::ocean::delete_ocean)

            .service(routes::sea::get_sea)
            .service(routes::sea::post_sea)
            .service(routes::sea::delete_sea)

            .service(routes::river::get_river)
            .service(routes::river::post_river)
            .service(routes::river::delete_river)
            .service(routes::river::update_river)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
