//! The version of the OpenAPI document: 2.2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterLeaderboardSchema {
    /// Character name.
    #[serde(rename = "name")]
    pub name: String,
    /// Character skin code.
    #[serde(rename = "skin")]
    pub skin: String,
    /// Achievements points.
    #[serde(rename = "achievements_points")]
    pub achievements_points: i32,
    /// Combat level.
    #[serde(rename = "level")]
    pub level: i32,
    /// Total XP of your character.
    #[serde(rename = "total_xp")]
    pub total_xp: i32,
    /// Mining level.
    #[serde(rename = "mining_level")]
    pub mining_level: i32,
    /// Mining total xp.
    #[serde(rename = "mining_total_xp")]
    pub mining_total_xp: i32,
    /// Woodcutting level.
    #[serde(rename = "woodcutting_level")]
    pub woodcutting_level: i32,
    /// Woodcutting total xp.
    #[serde(rename = "woodcutting_total_xp")]
    pub woodcutting_total_xp: i32,
    /// Fishing level.
    #[serde(rename = "fishing_level")]
    pub fishing_level: i32,
    /// Fishing total xp.
    #[serde(rename = "fishing_total_xp")]
    pub fishing_total_xp: i32,
    /// Weaponcrafting level.
    #[serde(rename = "weaponcrafting_level")]
    pub weaponcrafting_level: i32,
    /// Weaponcrafting total xp.
    #[serde(rename = "weaponcrafting_total_xp")]
    pub weaponcrafting_total_xp: i32,
    /// Gearcrafting level.
    #[serde(rename = "gearcrafting_level")]
    pub gearcrafting_level: i32,
    /// Gearcrafting total xp.
    #[serde(rename = "gearcrafting_total_xp")]
    pub gearcrafting_total_xp: i32,
    /// Jewelrycrafting level.
    #[serde(rename = "jewelrycrafting_level")]
    pub jewelrycrafting_level: i32,
    /// Jewelrycrafting total xp.
    #[serde(rename = "jewelrycrafting_total_xp")]
    pub jewelrycrafting_total_xp: i32,
    /// Cooking level.
    #[serde(rename = "cooking_level")]
    pub cooking_level: i32,
    /// Cooking total xp.
    #[serde(rename = "cooking_total_xp")]
    pub cooking_total_xp: i32,
    /// The numbers of golds on this character.
    #[serde(rename = "gold")]
    pub gold: i32,
}

impl CharacterLeaderboardSchema {
    pub fn new(
        name: String,
        skin: String,
        achievements_points: i32,
        level: i32,
        total_xp: i32,
        mining_level: i32,
        mining_total_xp: i32,
        woodcutting_level: i32,
        woodcutting_total_xp: i32,
        fishing_level: i32,
        fishing_total_xp: i32,
        weaponcrafting_level: i32,
        weaponcrafting_total_xp: i32,
        gearcrafting_level: i32,
        gearcrafting_total_xp: i32,
        jewelrycrafting_level: i32,
        jewelrycrafting_total_xp: i32,
        cooking_level: i32,
        cooking_total_xp: i32,
        gold: i32,
    ) -> CharacterLeaderboardSchema {
        CharacterLeaderboardSchema {
            name,
            skin,
            achievements_points,
            level,
            total_xp,
            mining_level,
            mining_total_xp,
            woodcutting_level,
            woodcutting_total_xp,
            fishing_level,
            fishing_total_xp,
            weaponcrafting_level,
            weaponcrafting_total_xp,
            gearcrafting_level,
            gearcrafting_total_xp,
            jewelrycrafting_level,
            jewelrycrafting_total_xp,
            cooking_level,
            cooking_total_xp,
            gold,
        }
    }
}
