use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

pub struct AppState {
    pub data: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub url: String,
}
