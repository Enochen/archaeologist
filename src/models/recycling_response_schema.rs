//! The version of the OpenAPI document: 2.2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecyclingResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::RecyclingDataSchema>,
}

impl RecyclingResponseSchema {
    pub fn new(data: models::RecyclingDataSchema) -> RecyclingResponseSchema {
        RecyclingResponseSchema {
            data: Box::new(data),
        }
    }
}
