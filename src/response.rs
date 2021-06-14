use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::GoalDataStatusKind;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TodoAppError {
  NoCapability,
  GoalIntentNonexistent,
  GoalNonexistent,
  TaskEventNonexistent,
  TimeUtilityFunctionNonexistent,
  TimeUtilityFunctionNotValid,
  NegativeStartTime,
  NegativeDuration,
  CannotAlterTask,
  NetworkError,
  DecodeError,
  InternalServerError,
  MethodNotAllowed,
  BadRequest,
  NotFound,
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
  pub goal_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentData {
    pub goal_intent_data_id: i64,
    pub creation_time: i64,
    pub creator: auth_service_api::response::User,
    pub name: String,
    pub active: bool
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
  pub goal_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub intent: Option<GoalIntent>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunction {
  pub time_utility_function_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunctionPoint {
  pub time_utility_function_point_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub time_utility_function: TimeUtilityFunction,
  pub start_time: i64,
  pub utils: i64,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalData {
  pub goal_data_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub goal: Goal,
  pub name: String,
  pub duration_estimate: i64,
  pub time_utility_function: TimeUtilityFunction,
  pub parent_goal: Option<Goal>,
  pub status: GoalDataStatusKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEvent {
  pub task_event_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub goal: Goal,
  pub start_time: i64,
  pub duration: i64,
  pub active: bool,
}
