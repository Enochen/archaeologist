//! The version of the OpenAPI document: 2.2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskTradeResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::TaskTradeDataSchema>,
}

impl TaskTradeResponseSchema {
    pub fn new(data: models::TaskTradeDataSchema) -> TaskTradeResponseSchema {
        TaskTradeResponseSchema {
            data: Box::new(data),
        }
    }
}
