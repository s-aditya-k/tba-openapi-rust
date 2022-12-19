/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventInsights2018 : Insights for FIRST Power Up qualification and elimination matches.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EventInsights2018 {
    /// An array with three values, number of times auto quest was completed, number of opportunities to complete the auto quest, and percentage.
    #[serde(rename = "auto_quest_achieved")]
    pub auto_quest_achieved: Vec<f32>,
    /// Average number of boost power up scored (out of 3).
    #[serde(rename = "average_boost_played")]
    pub average_boost_played: f32,
    /// Average endgame points.
    #[serde(rename = "average_endgame_points")]
    pub average_endgame_points: f32,
    /// Average number of force power up scored (out of 3).
    #[serde(rename = "average_force_played")]
    pub average_force_played: f32,
    /// Average foul score.
    #[serde(rename = "average_foul_score")]
    pub average_foul_score: f32,
    /// Average points scored during auto.
    #[serde(rename = "average_points_auto")]
    pub average_points_auto: f32,
    /// Average points scored during teleop.
    #[serde(rename = "average_points_teleop")]
    pub average_points_teleop: f32,
    /// Average mobility points scored during auto.
    #[serde(rename = "average_run_points_auto")]
    pub average_run_points_auto: f32,
    /// Average scale ownership points scored.
    #[serde(rename = "average_scale_ownership_points")]
    pub average_scale_ownership_points: f32,
    /// Average scale ownership points scored during auto.
    #[serde(rename = "average_scale_ownership_points_auto")]
    pub average_scale_ownership_points_auto: f32,
    /// Average scale ownership points scored during teleop.
    #[serde(rename = "average_scale_ownership_points_teleop")]
    pub average_scale_ownership_points_teleop: f32,
    /// Average score.
    #[serde(rename = "average_score")]
    pub average_score: f32,
    /// Average switch ownership points scored.
    #[serde(rename = "average_switch_ownership_points")]
    pub average_switch_ownership_points: f32,
    /// Average switch ownership points scored during auto.
    #[serde(rename = "average_switch_ownership_points_auto")]
    pub average_switch_ownership_points_auto: f32,
    /// Average switch ownership points scored during teleop.
    #[serde(rename = "average_switch_ownership_points_teleop")]
    pub average_switch_ownership_points_teleop: f32,
    /// Average value points scored.
    #[serde(rename = "average_vault_points")]
    pub average_vault_points: f32,
    /// Average margin of victory.
    #[serde(rename = "average_win_margin")]
    pub average_win_margin: f32,
    /// Average winning score.
    #[serde(rename = "average_win_score")]
    pub average_win_score: f32,
    /// An array with three values, number of times a boost power up was played, number of opportunities to play a boost power up, and percentage.
    #[serde(rename = "boost_played_counts")]
    pub boost_played_counts: Vec<f32>,
    /// An array with three values, number of times a climb occurred, number of opportunities to climb, and percentage.
    #[serde(rename = "climb_counts")]
    pub climb_counts: Vec<f32>,
    /// An array with three values, number of times an alliance faced the boss, number of opportunities to face the boss, and percentage.
    #[serde(rename = "face_the_boss_achieved")]
    pub face_the_boss_achieved: Vec<f32>,
    /// An array with three values, number of times a force power up was played, number of opportunities to play a force power up, and percentage.
    #[serde(rename = "force_played_counts")]
    pub force_played_counts: Vec<f32>,
    /// An array with three values, high score, match key from the match with the high score, and the name of the match
    #[serde(rename = "high_score")]
    pub high_score: Vec<String>,
    /// An array with three values, number of times a levitate power up was played, number of opportunities to play a levitate power up, and percentage.
    #[serde(rename = "levitate_played_counts")]
    pub levitate_played_counts: Vec<f32>,
    /// An array with three values, number of times a team scored mobility points in auto, number of opportunities to score mobility points in auto, and percentage.
    #[serde(rename = "run_counts_auto")]
    pub run_counts_auto: Vec<f32>,
    /// Average scale neutral percentage.
    #[serde(rename = "scale_neutral_percentage")]
    pub scale_neutral_percentage: f32,
    /// Average scale neutral percentage during auto.
    #[serde(rename = "scale_neutral_percentage_auto")]
    pub scale_neutral_percentage_auto: f32,
    /// Average scale neutral percentage during teleop.
    #[serde(rename = "scale_neutral_percentage_teleop")]
    pub scale_neutral_percentage_teleop: f32,
    /// An array with three values, number of times a switch was owned during auto, number of opportunities to own a switch during auto, and percentage.
    #[serde(rename = "switch_owned_counts_auto")]
    pub switch_owned_counts_auto: Vec<f32>,
    /// An array with three values, number of times a unicorn match (Win + Auto Quest + Face the Boss) occurred, number of opportunities to have a unicorn match, and percentage.
    #[serde(rename = "unicorn_matches")]
    pub unicorn_matches: Vec<f32>,
    /// Average opposing switch denail percentage for the winning alliance during teleop.
    #[serde(rename = "winning_opp_switch_denial_percentage_teleop")]
    pub winning_opp_switch_denial_percentage_teleop: f32,
    /// Average own switch ownership percentage for the winning alliance.
    #[serde(rename = "winning_own_switch_ownership_percentage")]
    pub winning_own_switch_ownership_percentage: f32,
    /// Average own switch ownership percentage for the winning alliance during auto.
    #[serde(rename = "winning_own_switch_ownership_percentage_auto")]
    pub winning_own_switch_ownership_percentage_auto: f32,
    /// Average own switch ownership percentage for the winning alliance during teleop.
    #[serde(rename = "winning_own_switch_ownership_percentage_teleop")]
    pub winning_own_switch_ownership_percentage_teleop: f32,
    /// Average scale ownership percentage for the winning alliance.
    #[serde(rename = "winning_scale_ownership_percentage")]
    pub winning_scale_ownership_percentage: f32,
    /// Average scale ownership percentage for the winning alliance during auto.
    #[serde(rename = "winning_scale_ownership_percentage_auto")]
    pub winning_scale_ownership_percentage_auto: f32,
    /// Average scale ownership percentage for the winning alliance during teleop.
    #[serde(rename = "winning_scale_ownership_percentage_teleop")]
    pub winning_scale_ownership_percentage_teleop: f32,
}

impl EventInsights2018 {
    /// Insights for FIRST Power Up qualification and elimination matches.
    pub fn new(auto_quest_achieved: Vec<f32>, average_boost_played: f32, average_endgame_points: f32, average_force_played: f32, average_foul_score: f32, average_points_auto: f32, average_points_teleop: f32, average_run_points_auto: f32, average_scale_ownership_points: f32, average_scale_ownership_points_auto: f32, average_scale_ownership_points_teleop: f32, average_score: f32, average_switch_ownership_points: f32, average_switch_ownership_points_auto: f32, average_switch_ownership_points_teleop: f32, average_vault_points: f32, average_win_margin: f32, average_win_score: f32, boost_played_counts: Vec<f32>, climb_counts: Vec<f32>, face_the_boss_achieved: Vec<f32>, force_played_counts: Vec<f32>, high_score: Vec<String>, levitate_played_counts: Vec<f32>, run_counts_auto: Vec<f32>, scale_neutral_percentage: f32, scale_neutral_percentage_auto: f32, scale_neutral_percentage_teleop: f32, switch_owned_counts_auto: Vec<f32>, unicorn_matches: Vec<f32>, winning_opp_switch_denial_percentage_teleop: f32, winning_own_switch_ownership_percentage: f32, winning_own_switch_ownership_percentage_auto: f32, winning_own_switch_ownership_percentage_teleop: f32, winning_scale_ownership_percentage: f32, winning_scale_ownership_percentage_auto: f32, winning_scale_ownership_percentage_teleop: f32) -> EventInsights2018 {
        EventInsights2018 {
            auto_quest_achieved,
            average_boost_played,
            average_endgame_points,
            average_force_played,
            average_foul_score,
            average_points_auto,
            average_points_teleop,
            average_run_points_auto,
            average_scale_ownership_points,
            average_scale_ownership_points_auto,
            average_scale_ownership_points_teleop,
            average_score,
            average_switch_ownership_points,
            average_switch_ownership_points_auto,
            average_switch_ownership_points_teleop,
            average_vault_points,
            average_win_margin,
            average_win_score,
            boost_played_counts,
            climb_counts,
            face_the_boss_achieved,
            force_played_counts,
            high_score,
            levitate_played_counts,
            run_counts_auto,
            scale_neutral_percentage,
            scale_neutral_percentage_auto,
            scale_neutral_percentage_teleop,
            switch_owned_counts_auto,
            unicorn_matches,
            winning_opp_switch_denial_percentage_teleop,
            winning_own_switch_ownership_percentage,
            winning_own_switch_ownership_percentage_auto,
            winning_own_switch_ownership_percentage_teleop,
            winning_scale_ownership_percentage,
            winning_scale_ownership_percentage_auto,
            winning_scale_ownership_percentage_teleop,
        }
    }
}


