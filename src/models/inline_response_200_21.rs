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
pub struct InlineResponse20021 {
    /// account_id
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,
    /// avatarfull
    #[serde(rename = "avatarfull", skip_serializing_if = "Option::is_none")]
    pub avatarfull: Option<String>,
    /// personaname
    #[serde(rename = "personaname", skip_serializing_if = "Option::is_none")]
    pub personaname: Option<String>,
    /// last_match_time. May not be present or null.
    #[serde(rename = "last_match_time", skip_serializing_if = "Option::is_none")]
    pub last_match_time: Option<String>,
    /// similarity
    #[serde(rename = "similarity", skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

impl InlineResponse20021 {
    pub fn new() -> InlineResponse20021 {
        InlineResponse20021 {
            account_id: None,
            avatarfull: None,
            personaname: None,
            last_match_time: None,
            similarity: None,
        }
    }
}


