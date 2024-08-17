mod handlers;
mod util;
mod models;

use models::{apiresponse::APIResponse,appstate::AppState};
use handlers::apihandler::{availablelinks,deeplink};
use util::json_loader::load_json;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use actix_cors::Cors;



#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(APIResponse {
        deeplink: None,
        message: Some("Health OK!".to_string()),
    })
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::Ok().json(APIResponse {
        deeplink: None,
        message: Some("Information about this API".to_string()),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = load_json().expect("Failed to load JSON file");

    let app_state = web::Data::new(AppState {
        data: Arc::new(Mutex::new(data)),
    });

    println!("Server started");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET"])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            )
            .service(health)
            .service(info)          
            .service((
                web::resource("/availablelinks")
                    .route(web::get().to(availablelinks)),
                web::resource("/")
                    .route(web::get().to(deeplink))
            ))  // Using `.service(health)` directly because of `#[get]` macro
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}