/*
 * OpenDota API
 *
 * # Introduction The OpenDota API provides Dota 2 related data including advanced match data extracted from match replays.  You can find data that can be used to convert hero and ability IDs and other information provided by the API from the [dotaconstants](https://github.com/odota/dotaconstants) repository.  **Beginning 2018-04-22, the OpenDota API is limited to 50,000 free calls per month and 60 requests/minute** We offer a Premium Tier with unlimited API calls and higher rate limits. Check out the [API page](https://www.opendota.com/api-keys) to learn more. 
 *
 * The version of the OpenAPI document: 18.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InlineResponse2004 : match



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    /// Match ID
    #[serde(rename = "match_id", skip_serializing_if = "Option::is_none")]
    pub match_id: Option<i32>,
    /// Which slot the player is in. 0-127 are Radiant, 128-255 are Dire
    #[serde(rename = "player_slot", skip_serializing_if = "Option::is_none")]
    pub player_slot: Option<i32>,
    /// Boolean indicating whether Radiant won the match
    #[serde(rename = "radiant_win", skip_serializing_if = "Option::is_none")]
    pub radiant_win: Option<bool>,
    /// Duration of the game in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Integer corresponding to game mode played. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/game_mode.json
    #[serde(rename = "game_mode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<i32>,
    /// Integer corresponding to lobby type of match. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/lobby_type.json
    #[serde(rename = "lobby_type", skip_serializing_if = "Option::is_none")]
    pub lobby_type: Option<i32>,
    /// The ID value of the hero played
    #[serde(rename = "hero_id", skip_serializing_if = "Option::is_none")]
    pub hero_id: Option<i32>,
    /// Start time of the match in seconds elapsed since 1970
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i32>,
    /// version
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// Total kills the player had at the end of the match
    #[serde(rename = "kills", skip_serializing_if = "Option::is_none")]
    pub kills: Option<i32>,
    /// Total deaths the player had at the end of the match
    #[serde(rename = "deaths", skip_serializing_if = "Option::is_none")]
    pub deaths: Option<i32>,
    /// Total assists the player had at the end of the match
    #[serde(rename = "assists", skip_serializing_if = "Option::is_none")]
    pub assists: Option<i32>,
    /// Skill bracket assigned by Valve (Normal, High, Very High). If the skill is unknown, will return null.
    #[serde(rename = "skill", skip_serializing_if = "Option::is_none")]
    pub skill: Option<i32>,
    /// Experience Per Minute obtained by the player
    #[serde(rename = "xp_per_min", skip_serializing_if = "Option::is_none")]
    pub xp_per_min: Option<i32>,
    /// Average gold per minute of the player
    #[serde(rename = "gold_per_min", skip_serializing_if = "Option::is_none")]
    pub gold_per_min: Option<i32>,
    /// Total hero damage to enemy heroes
    #[serde(rename = "hero_damage", skip_serializing_if = "Option::is_none")]
    pub hero_damage: Option<i32>,
    /// Total healing of ally heroes
    #[serde(rename = "hero_healing", skip_serializing_if = "Option::is_none")]
    pub hero_healing: Option<i32>,
    /// Total last hits the player had at the end of the match
    #[serde(rename = "last_hits", skip_serializing_if = "Option::is_none")]
    pub last_hits: Option<i32>,
    /// Integer corresponding to which lane the player laned in for the match
    #[serde(rename = "lane", skip_serializing_if = "Option::is_none")]
    pub lane: Option<i32>,
    /// lane_role
    #[serde(rename = "lane_role", skip_serializing_if = "Option::is_none")]
    pub lane_role: Option<i32>,
    /// Boolean describing whether or not the player roamed
    #[serde(rename = "is_roaming", skip_serializing_if = "Option::is_none")]
    pub is_roaming: Option<bool>,
    /// cluster
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<i32>,
    /// Integer describing whether or not the player left the game. 0: didn't leave. 1: left safely. 2+: Abandoned
    #[serde(rename = "leaver_status", skip_serializing_if = "Option::is_none")]
    pub leaver_status: Option<i32>,
    /// Size of the players party. If not in a party, will return 1.
    #[serde(rename = "party_size", skip_serializing_if = "Option::is_none")]
    pub party_size: Option<i32>,
}

impl InlineResponse2004 {
    /// match
    pub fn new() -> InlineResponse2004 {
        InlineResponse2004 {
            match_id: None,
            player_slot: None,
            radiant_win: None,
            duration: None,
            game_mode: None,
            lobby_type: None,
            hero_id: None,
            start_time: None,
            version: None,
            kills: None,
            deaths: None,
            assists: None,
            skill: None,
            xp_per_min: None,
            gold_per_min: None,
            hero_damage: None,
            hero_healing: None,
            last_hits: None,
            lane: None,
            lane_role: None,
            is_roaming: None,
            cluster: None,
            leaver_status: None,
            party_size: None,
        }
    }
}


