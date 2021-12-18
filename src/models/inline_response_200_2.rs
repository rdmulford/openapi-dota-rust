/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2002 {
    /// tracked_until
    #[serde(rename = "tracked_until", skip_serializing_if = "Option::is_none")]
    pub tracked_until: Option<String>,
    /// solo_competitive_rank
    #[serde(rename = "solo_competitive_rank", skip_serializing_if = "Option::is_none")]
    pub solo_competitive_rank: Option<String>,
    /// competitive_rank
    #[serde(rename = "competitive_rank", skip_serializing_if = "Option::is_none")]
    pub competitive_rank: Option<String>,
    /// rank_tier
    #[serde(rename = "rank_tier", skip_serializing_if = "Option::is_none")]
    pub rank_tier: Option<f32>,
    /// leaderboard_rank
    #[serde(rename = "leaderboard_rank", skip_serializing_if = "Option::is_none")]
    pub leaderboard_rank: Option<f32>,
    #[serde(rename = "mmr_estimate", skip_serializing_if = "Option::is_none")]
    pub mmr_estimate: Option<Box<crate::models::InlineResponse2002MmrEstimate>>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::InlineResponse2002Profile>>,
}

impl InlineResponse2002 {
    pub fn new() -> InlineResponse2002 {
        InlineResponse2002 {
            tracked_until: None,
            solo_competitive_rank: None,
            competitive_rank: None,
            rank_tier: None,
            leaderboard_rank: None,
            mmr_estimate: None,
            profile: None,
        }
    }
}


