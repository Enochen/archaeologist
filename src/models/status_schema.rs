/*
 * Artifacts API
 *
 *  Artifacts is an API-based MMO game where you can manage 5 characters to explore, fight, gather resources, craft items and much more.  Website: https://artifactsmmo.com/  Documentation: https://docs.artifactsmmo.com/  OpenAPI Spec: https://api.artifactsmmo.com/openapi.json
 *
 * The version of the OpenAPI document: 1.6
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusSchema {
    /// Server status
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "characters_online", skip_serializing_if = "Option::is_none")]
    pub characters_online: Option<i32>,
    #[serde(rename = "server_time", skip_serializing_if = "Option::is_none")]
    pub server_time: Option<String>,
    #[serde(rename = "announcements", skip_serializing_if = "Option::is_none")]
    pub announcements: Option<Vec<models::AnnouncementSchema>>,
    /// Last server wipe.
    #[serde(rename = "last_wipe")]
    pub last_wipe: String,
    /// Next server wipe.
    #[serde(rename = "next_wipe")]
    pub next_wipe: String,
}

impl StatusSchema {
    pub fn new(status: String, last_wipe: String, next_wipe: String) -> StatusSchema {
        StatusSchema {
            status,
            version: None,
            characters_online: None,
            server_time: None,
            announcements: None,
            last_wipe,
            next_wipe,
        }
    }
}
