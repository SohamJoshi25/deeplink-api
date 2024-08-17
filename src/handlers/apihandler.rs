use crate::models::{appstate::{AppState, QueryParams}, apiresponse::{APIResponse, Link}};
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
// Define the ASCII set for URL encoding

fn extract_app_name(url: &str) -> &str {
    let parts: Vec<&str> = url.split('.').collect();
    if parts.len() > 1 {
        parts[1]
    } else {
        url
    }
}

pub async fn deeplink(
    params: web::Query<QueryParams>,
    state: web::Data<AppState>,
) -> impl Responder {
    let link = params.url.clone();
    let packages: std::sync::MutexGuard<HashMap<String, String>> = state.data.lock().unwrap();

    let base_url: &str = if link.starts_with("https://") {
        &link[8..]
    } else {
        &link
    };


    let app = extract_app_name(&base_url);

    println!("{}", app);

    let package = match packages.get(app) {
        Some(p) => p.clone(),
        None => {
            return HttpResponse::BadRequest().json(APIResponse {
                deeplink: None,
                message: Some("Link not recognized".to_string()),
            });
        }
    };

    let deepurl_android = format!("intent://{}#Intent;scheme=https;package={};end", base_url, package);
    let deepurl_ios = ""; // Handle iOS logic if needed

    let response = APIResponse {
        deeplink: Some(Link {
            android: deepurl_android.replace(" ", "+"),
            ios: deepurl_ios.to_string(),
        }),
        message: Some("Success".to_string()),
    };

    HttpResponse::Ok().json(response)
}

pub async fn availablelinks(state: web::Data<AppState>) -> impl Responder {
    // Retrieve the entire HashMap from AppState
    let packages = state.data.lock().unwrap();

    // Convert the HashMap into a JSON-compatible format
    let response = packages.iter().map(|(key, value)| {
        (key.clone(), value.clone())
    }).collect::<HashMap<String, String>>();

    // Return the data as JSON
    HttpResponse::Ok().json(response)
}
