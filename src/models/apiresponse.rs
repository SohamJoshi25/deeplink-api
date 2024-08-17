use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse{
    pub deeplink:Option<Link>,
    pub message:Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct APIRequest{
    pub link:Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link{
    pub android:String,
    pub ios:String,
}