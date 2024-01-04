pub mod result;
pub mod user;
pub mod organization;
pub mod org_member;
pub mod org_job;
pub mod schedule;

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataJson<T> where T: Serialize {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataList<T> where T: Serialize {
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseDataMessage {
    pub message: String,
}
