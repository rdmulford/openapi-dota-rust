# InlineResponse200

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_id** | Option<**i32**> | The ID number of the match assigned by Valve | [optional]
**barracks_status_dire** | Option<**i32**> | Bitmask. An integer that represents a binary of which barracks are still standing. 63 would mean all barracks still stand at the end of the game. | [optional]
**barracks_status_radiant** | Option<**i32**> | Bitmask. An integer that represents a binary of which barracks are still standing. 63 would mean all barracks still stand at the end of the game. | [optional]
**chat** | Option<[**Vec<crate::models::InlineResponse200Chat>**](inline_response_200_chat.md)> | Array containing information on the chat of the game | [optional]
**cluster** | Option<**i32**> | cluster | [optional]
**cosmetics** | Option<[**serde_json::Value**](.md)> | cosmetics | [optional]
**dire_score** | Option<**i32**> | Final score for Dire (number of kills on Radiant) | [optional]
**draft_timings** | Option<[**Vec<crate::models::InlineResponse200DraftTimings>**](inline_response_200_draft_timings.md)> | draft_timings | [optional]
**duration** | Option<**i32**> | Duration of the game in seconds | [optional]
**engine** | Option<**i32**> | engine | [optional]
**first_blood_time** | Option<**i32**> | Time in seconds at which first blood occurred | [optional]
**game_mode** | Option<**i32**> | Integer corresponding to game mode played. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/game_mode.json | [optional]
**human_players** | Option<**i32**> | Number of human players in the game | [optional]
**leagueid** | Option<**i32**> | leagueid | [optional]
**lobby_type** | Option<**i32**> | Integer corresponding to lobby type of match. List of constants can be found here: https://github.com/odota/dotaconstants/blob/master/json/lobby_type.json | [optional]
**match_seq_num** | Option<**i32**> | match_seq_num | [optional]
**negative_votes** | Option<**i32**> | Number of negative votes the replay received in the in-game client | [optional]
**objectives** | Option<[**serde_json::Value**](.md)> | objectives | [optional]
**picks_bans** | Option<[**serde_json::Value**](.md)> | Object containing information on the draft. Each pick/ban contains a boolean relating to whether the choice is a pick or a ban, the hero ID, the team the picked or banned it, and the order. | [optional]
**positive_votes** | Option<**i32**> | Number of positive votes the replay received in the in-game client | [optional]
**radiant_gold_adv** | Option<[**serde_json::Value**](.md)> | Array of the Radiant gold advantage at each minute in the game. A negative number means that Radiant is behind, and thus it is their gold disadvantage.  | [optional]
**radiant_score** | Option<**i32**> | Final score for Radiant (number of kills on Radiant) | [optional]
**radiant_win** | Option<**bool**> | Boolean indicating whether Radiant won the match | [optional]
**radiant_xp_adv** | Option<[**serde_json::Value**](.md)> | Array of the Radiant experience advantage at each minute in the game. A negative number means that Radiant is behind, and thus it is their experience disadvantage.  | [optional]
**start_time** | Option<**i32**> | The Unix timestamp at which the game started | [optional]
**teamfights** | Option<[**serde_json::Value**](.md)> | teamfights | [optional]
**tower_status_dire** | Option<**i32**> | Bitmask. An integer that represents a binary of which Dire towers are still standing. | [optional]
**tower_status_radiant** | Option<**i32**> | Bitmask. An integer that represents a binary of which Radiant towers are still standing. | [optional]
**version** | Option<**i32**> | Parse version, used internally by OpenDota | [optional]
**replay_salt** | Option<**i32**> | replay_salt | [optional]
**series_id** | Option<**i32**> | series_id | [optional]
**series_type** | Option<**i32**> | series_type | [optional]
**radiant_team** | Option<[**serde_json::Value**](.md)> | radiant_team | [optional]
**dire_team** | Option<[**serde_json::Value**](.md)> | dire_team | [optional]
**league** | Option<[**serde_json::Value**](.md)> | league | [optional]
**skill** | Option<**i32**> | Skill bracket assigned by Valve (Normal, High, Very High) | [optional]
**players** | Option<[**Vec<crate::models::InlineResponse200Players>**](inline_response_200_players.md)> | Array of information on individual players | [optional]
**patch** | Option<**i32**> | Information on the patch version the game is played on | [optional]
**region** | Option<**i32**> | Integer corresponding to the region the game was played on | [optional]
**all_word_counts** | Option<[**serde_json::Value**](.md)> | Word counts of the all chat messages in the player's games | [optional]
**my_word_counts** | Option<[**serde_json::Value**](.md)> | Word counts of the player's all chat messages | [optional]
**throw** | Option<**i32**> | Maximum gold advantage of the player's team if they lost the match | [optional]
**comeback** | Option<**i32**> | Maximum gold disadvantage of the player's team if they won the match | [optional]
**loss** | Option<**i32**> | Maximum gold disadvantage of the player's team if they lost the match | [optional]
**win** | Option<**i32**> | Maximum gold advantage of the player's team if they won the match | [optional]
**replay_url** | Option<**String**> | replay_url | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


