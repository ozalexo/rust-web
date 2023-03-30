use crate::{error::Error, state::AppState};
use rocket::State;
use rocket::serde::{json::Json};
use crate::{models::{AuthData, RefundData, ResponseMessage, Records}};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/authorize", format = "json", data = "<data>")]
pub fn authorize(state: &State<AppState>, data: Json<AuthData>) -> Result<Json<ResponseMessage>, Error> {
    let records = state.service.authorize(data)?;
    let response = Json(ResponseMessage{message: "Test".to_string()});
    Ok(response)
}

#[post("/refund", format = "json", data = "<data>")]
pub fn refund(state: &State<AppState>, data: Json<RefundData>) -> Result<Json<ResponseMessage>, Error> {
    let records = state.service.refund(data)?;
    let response = Json(ResponseMessage{message: "Test".to_string()});
    Ok(response)
}

#[get("/queue?<page>&<limit>&<last_page>")]
pub fn queue(state: &State<AppState>, page: Option<usize>, limit: Option<usize>, last_page: Option<usize>) -> Result<Json<Records>, Error> {
    let records = state.service.queue(page, limit, last_page)?;
    let response = Json(records);
    Ok(response)
}