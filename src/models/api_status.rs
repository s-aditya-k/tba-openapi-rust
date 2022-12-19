/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiStatus {
    /// Year of the current FRC season.
    #[serde(rename = "current_season")]
    pub current_season: i32,
    /// Maximum FRC season year for valid queries.
    #[serde(rename = "max_season")]
    pub max_season: i32,
    /// True if the entire FMS API provided by FIRST is down.
    #[serde(rename = "is_datafeed_down")]
    pub is_datafeed_down: bool,
    /// An array of strings containing event keys of any active events that are no longer updating.
    #[serde(rename = "down_events")]
    pub down_events: Vec<String>,
    #[serde(rename = "ios")]
    pub ios: Box<crate::models::ApiStatusAppVersion>,
    #[serde(rename = "android")]
    pub android: Box<crate::models::ApiStatusAppVersion>,
}

impl ApiStatus {
    pub fn new(current_season: i32, max_season: i32, is_datafeed_down: bool, down_events: Vec<String>, ios: crate::models::ApiStatusAppVersion, android: crate::models::ApiStatusAppVersion) -> ApiStatus {
        ApiStatus {
            current_season,
            max_season,
            is_datafeed_down,
            down_events,
            ios: Box::new(ios),
            android: Box::new(android),
        }
    }
}


