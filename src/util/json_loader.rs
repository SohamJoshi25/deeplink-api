use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde_json;

pub fn load_json() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let data = fs::read_to_string("resources/packages.json")?;
    let parsed: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(parsed)
}