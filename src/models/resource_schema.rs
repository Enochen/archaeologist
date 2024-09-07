//! The version of the OpenAPI document: 2.1
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSchema {
    /// The name of the resource
    #[serde(rename = "name")]
    pub name: String,
    /// The code of the resource. This is the resource's unique identifier (ID).
    #[serde(rename = "code")]
    pub code: String,
    /// The skill required to gather this resource.
    #[serde(rename = "skill")]
    pub skill: Skill,
    /// The skill level required to gather this resource.
    #[serde(rename = "level")]
    pub level: i32,
    /// The drops of this resource.
    #[serde(rename = "drops")]
    pub drops: Vec<models::DropRateSchema>,
}

impl ResourceSchema {
    pub fn new(
        name: String,
        code: String,
        skill: Skill,
        level: i32,
        drops: Vec<models::DropRateSchema>,
    ) -> ResourceSchema {
        ResourceSchema {
            name,
            code,
            skill,
            level,
            drops,
        }
    }
}
/// The skill required to gather this resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Skill {
    #[serde(rename = "mining")]
    Mining,
    #[serde(rename = "woodcutting")]
    Woodcutting,
    #[serde(rename = "fishing")]
    Fishing,
}

impl Default for Skill {
    fn default() -> Skill {
        Self::Mining
    }
}
