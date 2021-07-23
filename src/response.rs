use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::GoalDataStatusKind;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TodoAppError {
  NoCapability,
  GoalIntentNonexistent,
  GoalNonexistent,
  ExternalEventNonexistent,
  TaskEventNonexistent,
  TimeUtilityFunctionNonexistent,
  TimeUtilityFunctionNotValid,
  NegativeStartTime,
  NegativeDuration,
  GoalFormsCycle,
  CannotAlterTask,
  DecodeError,
  InternalServerError,
  MethodNotAllowed,
  Unauthorized,
  BadRequest,
  NotFound,
  Network,
  Unknown,
}

impl std::fmt::Display for TodoAppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}

impl std::error::Error for TodoAppError {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntent {
  pub goal_intent_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentData {
    pub goal_intent_data_id: i64,
    pub creation_time: i64,
    pub creator_user_id: i64,
    pub goal_intent: GoalIntent,
    pub name: String,
    pub active: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
  pub goal_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub intent: Option<GoalIntent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunction {
  pub time_utility_function_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub start_times: Vec<i64>,
  pub utils: Vec<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalData {
  pub goal_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub goal: Goal,
  pub name: String,
  pub tags: Vec<String>,
  pub duration_estimate: i64,
  pub time_utility_function: TimeUtilityFunction,
  pub time_span: Option<(i64, i64)>,
  pub parent_goal: Option<Goal>,
  pub status: GoalDataStatusKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEvent {
  pub external_event_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEventData {
  pub external_event_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub external_event: ExternalEvent,
  pub name: String,
  pub start_time: i64,
  pub end_time: i64,
  pub active: bool,
}
