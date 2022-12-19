/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchScoreBreakdown2017 : See the 2017 FMS API documentation for a description of each value.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2017 {
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<Box<crate::models::MatchScoreBreakdown2017Alliance>>,
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<Box<crate::models::MatchScoreBreakdown2017Alliance>>,
}

impl MatchScoreBreakdown2017 {
    /// See the 2017 FMS API documentation for a description of each value.
    pub fn new() -> MatchScoreBreakdown2017 {
        MatchScoreBreakdown2017 {
            blue: None,
            red: None,
        }
    }
}


