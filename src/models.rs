use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMessage {
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub username: String,
    pub funds: usize,
}

#[derive(Debug, Serialize)]
pub struct Records {
    pub records: Vec<Record>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthData {
    pub account: String,
    pub amount: u32,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RefundData {
    pub account: String,
    pub amount: u32,
}