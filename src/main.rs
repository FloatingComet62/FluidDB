use actix_web::{HttpServer, App};
use fluid_api::routes::{home, ocean, sea, river};

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home)

            .service(ocean::get_ocean)
            .service(ocean::post_ocean)
            .service(ocean::delete_ocean)

            .service(sea::get_sea)
            .service(sea::post_sea)
            .service(sea::delete_sea)

            .service(river::get_river)
            .service(river::post_river)
            .service(river::delete_river)
            .service(river::update_river)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
