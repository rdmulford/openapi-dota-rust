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
pub struct InlineResponse20020RanksRows {
    /// bin
    #[serde(rename = "bin", skip_serializing_if = "Option::is_none")]
    pub bin: Option<i32>,
    /// bin_name
    #[serde(rename = "bin_name", skip_serializing_if = "Option::is_none")]
    pub bin_name: Option<i32>,
    /// count
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// cumulative_sum
    #[serde(rename = "cumulative_sum", skip_serializing_if = "Option::is_none")]
    pub cumulative_sum: Option<i32>,
}

impl InlineResponse20020RanksRows {
    pub fn new() -> InlineResponse20020RanksRows {
        InlineResponse20020RanksRows {
            bin: None,
            bin_name: None,
            count: None,
            cumulative_sum: None,
        }
    }
}


