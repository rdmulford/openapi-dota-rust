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
pub struct InlineResponse20038 {
    /// table_name
    #[serde(rename = "table_name", skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// column_name
    #[serde(rename = "column_name", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    /// data_type
    #[serde(rename = "data_type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
}

impl InlineResponse20038 {
    pub fn new() -> InlineResponse20038 {
        InlineResponse20038 {
            table_name: None,
            column_name: None,
            data_type: None,
        }
    }
}


