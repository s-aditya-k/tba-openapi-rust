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
pub struct MatchScoreBreakdown2017Alliance {
    #[serde(rename = "autoPoints", skip_serializing_if = "Option::is_none")]
    pub auto_points: Option<i32>,
    #[serde(rename = "teleopPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_points: Option<i32>,
    #[serde(rename = "foulPoints", skip_serializing_if = "Option::is_none")]
    pub foul_points: Option<i32>,
    #[serde(rename = "adjustPoints", skip_serializing_if = "Option::is_none")]
    pub adjust_points: Option<i32>,
    #[serde(rename = "totalPoints", skip_serializing_if = "Option::is_none")]
    pub total_points: Option<i32>,
    #[serde(rename = "robot1Auto", skip_serializing_if = "Option::is_none")]
    pub robot1_auto: Option<Robot1Auto>,
    #[serde(rename = "robot2Auto", skip_serializing_if = "Option::is_none")]
    pub robot2_auto: Option<Robot2Auto>,
    #[serde(rename = "robot3Auto", skip_serializing_if = "Option::is_none")]
    pub robot3_auto: Option<Robot3Auto>,
    #[serde(rename = "rotor1Auto", skip_serializing_if = "Option::is_none")]
    pub rotor1_auto: Option<bool>,
    #[serde(rename = "rotor2Auto", skip_serializing_if = "Option::is_none")]
    pub rotor2_auto: Option<bool>,
    #[serde(rename = "autoFuelLow", skip_serializing_if = "Option::is_none")]
    pub auto_fuel_low: Option<i32>,
    #[serde(rename = "autoFuelHigh", skip_serializing_if = "Option::is_none")]
    pub auto_fuel_high: Option<i32>,
    #[serde(rename = "autoMobilityPoints", skip_serializing_if = "Option::is_none")]
    pub auto_mobility_points: Option<i32>,
    #[serde(rename = "autoRotorPoints", skip_serializing_if = "Option::is_none")]
    pub auto_rotor_points: Option<i32>,
    #[serde(rename = "autoFuelPoints", skip_serializing_if = "Option::is_none")]
    pub auto_fuel_points: Option<i32>,
    #[serde(rename = "teleopFuelPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_fuel_points: Option<i32>,
    #[serde(rename = "teleopFuelLow", skip_serializing_if = "Option::is_none")]
    pub teleop_fuel_low: Option<i32>,
    #[serde(rename = "teleopFuelHigh", skip_serializing_if = "Option::is_none")]
    pub teleop_fuel_high: Option<i32>,
    #[serde(rename = "teleopRotorPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_rotor_points: Option<i32>,
    #[serde(rename = "kPaRankingPointAchieved", skip_serializing_if = "Option::is_none")]
    pub k_pa_ranking_point_achieved: Option<bool>,
    #[serde(rename = "teleopTakeoffPoints", skip_serializing_if = "Option::is_none")]
    pub teleop_takeoff_points: Option<i32>,
    #[serde(rename = "kPaBonusPoints", skip_serializing_if = "Option::is_none")]
    pub k_pa_bonus_points: Option<i32>,
    #[serde(rename = "rotorBonusPoints", skip_serializing_if = "Option::is_none")]
    pub rotor_bonus_points: Option<i32>,
    #[serde(rename = "rotor1Engaged", skip_serializing_if = "Option::is_none")]
    pub rotor1_engaged: Option<bool>,
    #[serde(rename = "rotor2Engaged", skip_serializing_if = "Option::is_none")]
    pub rotor2_engaged: Option<bool>,
    #[serde(rename = "rotor3Engaged", skip_serializing_if = "Option::is_none")]
    pub rotor3_engaged: Option<bool>,
    #[serde(rename = "rotor4Engaged", skip_serializing_if = "Option::is_none")]
    pub rotor4_engaged: Option<bool>,
    #[serde(rename = "rotorRankingPointAchieved", skip_serializing_if = "Option::is_none")]
    pub rotor_ranking_point_achieved: Option<bool>,
    #[serde(rename = "techFoulCount", skip_serializing_if = "Option::is_none")]
    pub tech_foul_count: Option<i32>,
    #[serde(rename = "foulCount", skip_serializing_if = "Option::is_none")]
    pub foul_count: Option<i32>,
    #[serde(rename = "touchpadNear", skip_serializing_if = "Option::is_none")]
    pub touchpad_near: Option<String>,
    #[serde(rename = "touchpadMiddle", skip_serializing_if = "Option::is_none")]
    pub touchpad_middle: Option<String>,
    #[serde(rename = "touchpadFar", skip_serializing_if = "Option::is_none")]
    pub touchpad_far: Option<String>,
}

impl MatchScoreBreakdown2017Alliance {
    pub fn new() -> MatchScoreBreakdown2017Alliance {
        MatchScoreBreakdown2017Alliance {
            auto_points: None,
            teleop_points: None,
            foul_points: None,
            adjust_points: None,
            total_points: None,
            robot1_auto: None,
            robot2_auto: None,
            robot3_auto: None,
            rotor1_auto: None,
            rotor2_auto: None,
            auto_fuel_low: None,
            auto_fuel_high: None,
            auto_mobility_points: None,
            auto_rotor_points: None,
            auto_fuel_points: None,
            teleop_fuel_points: None,
            teleop_fuel_low: None,
            teleop_fuel_high: None,
            teleop_rotor_points: None,
            k_pa_ranking_point_achieved: None,
            teleop_takeoff_points: None,
            k_pa_bonus_points: None,
            rotor_bonus_points: None,
            rotor1_engaged: None,
            rotor2_engaged: None,
            rotor3_engaged: None,
            rotor4_engaged: None,
            rotor_ranking_point_achieved: None,
            tech_foul_count: None,
            foul_count: None,
            touchpad_near: None,
            touchpad_middle: None,
            touchpad_far: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Robot1Auto {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Mobility")]
    Mobility,
    #[serde(rename = "None")]
    None,
}

impl Default for Robot1Auto {
    fn default() -> Robot1Auto {
        Self::Unknown
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Robot2Auto {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Mobility")]
    Mobility,
    #[serde(rename = "None")]
    None,
}

impl Default for Robot2Auto {
    fn default() -> Robot2Auto {
        Self::Unknown
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Robot3Auto {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Mobility")]
    Mobility,
    #[serde(rename = "None")]
    None,
}

impl Default for Robot3Auto {
    fn default() -> Robot3Auto {
        Self::Unknown
    }
}

