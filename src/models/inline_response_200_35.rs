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
pub struct InlineResponse20035 {
    /// Hero ID
    #[serde(rename = "hero_id", skip_serializing_if = "Option::is_none")]
    pub hero_id: Option<i32>,
    /// Purchased item
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    /// Ingame time in seconds before the item was purchased
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
    /// The number of games where the hero bought this item before this time
    #[serde(rename = "games", skip_serializing_if = "Option::is_none")]
    pub games: Option<String>,
    /// The number of games won where the hero bought this item before this time
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<String>,
}

impl InlineResponse20035 {
    pub fn new() -> InlineResponse20035 {
        InlineResponse20035 {
            hero_id: None,
            item: None,
            time: None,
            games: None,
            wins: None,
        }
    }
}


