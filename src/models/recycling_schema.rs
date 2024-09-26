//! The version of the OpenAPI document: 2.2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecyclingSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Quantity of items to recycle.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

impl RecyclingSchema {
    pub fn new(code: String) -> RecyclingSchema {
        RecyclingSchema {
            code,
            quantity: None,
        }
    }
}
