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
pub struct InlineResponse20026 {
    /// Numeric identifier for the hero object
    #[serde(rename = "hero_id", skip_serializing_if = "Option::is_none")]
    pub hero_id: Option<i32>,
    /// Number of games played
    #[serde(rename = "games_played", skip_serializing_if = "Option::is_none")]
    pub games_played: Option<i32>,
    /// Number of games won
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,
}

impl InlineResponse20026 {
    pub fn new() -> InlineResponse20026 {
        InlineResponse20026 {
            hero_id: None,
            games_played: None,
            wins: None,
        }
    }
}


