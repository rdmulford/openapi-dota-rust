/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse200DraftTimings : draft_stage



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse200DraftTimings {
    /// order
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// pick
    #[serde(rename = "pick", skip_serializing_if = "Option::is_none")]
    pub pick: Option<bool>,
    /// active_team
    #[serde(rename = "active_team", skip_serializing_if = "Option::is_none")]
    pub active_team: Option<i32>,
    /// The ID value of the hero played
    #[serde(rename = "hero_id", skip_serializing_if = "Option::is_none")]
    pub hero_id: Option<i32>,
    /// Which slot the player is in. 0-127 are Radiant, 128-255 are Dire
    #[serde(rename = "player_slot", skip_serializing_if = "Option::is_none")]
    pub player_slot: Option<i32>,
    /// extra_time
    #[serde(rename = "extra_time", skip_serializing_if = "Option::is_none")]
    pub extra_time: Option<i32>,
    /// total_time_taken
    #[serde(rename = "total_time_taken", skip_serializing_if = "Option::is_none")]
    pub total_time_taken: Option<i32>,
}

impl InlineResponse200DraftTimings {
    /// draft_stage
    pub fn new() -> InlineResponse200DraftTimings {
        InlineResponse200DraftTimings {
            order: None,
            pick: None,
            active_team: None,
            hero_id: None,
            player_slot: None,
            extra_time: None,
            total_time_taken: None,
        }
    }
}


