use crate::{models::{Record, Records, ResponseMessage, AuthData, RefundData}, error::Error};
use rocket::serde::{json::Json};

#[derive(Debug)]
pub struct Service {}

const DEFAULT_PAGE: usize = 1;
const DEFAULT_LIMIT: usize = 10;
const DEFAULT_LAST_PAGE: usize = 1;

impl Service {
    pub fn new() -> Service {
        Service {}
    }

    pub fn authorize(&self, data: Json<AuthData>) -> Result<ResponseMessage, Error> {
        return Err(Error::InvalidRefund("invalid authorize".to_string()));
    }

    pub fn refund(&self, data: Json<RefundData>) -> Result<ResponseMessage, Error> {
        return Err(Error::InsufficientFunds("invalid refund".to_string()));
    }

    pub fn queue(&self, page: Option<usize>, limit: Option<usize>, last_page: Option<usize>) -> Result<Records, Error> {
        let page = page.unwrap_or(DEFAULT_PAGE);
        let limit = limit.unwrap_or(DEFAULT_LIMIT);
        let last_page = last_page.unwrap_or(DEFAULT_LAST_PAGE);

        let mut records = Vec::new();
        records.push(Record{
            username: "John".to_string(),
            funds: 33,
        });

        // Ok(records)

        return Err(Error::Internal("invalid queue".to_string()))
    }
}
