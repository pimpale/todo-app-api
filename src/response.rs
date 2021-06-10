use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::GoalDataStatusKind;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TodoAppError {
  NoCapability,
  GoalNonexistent,
  PastEventNonexistent,
  TimeUtilityFunctionNonexistent,
  TimeUtilityFunctionNotValid,
  NegativeStartTime,
  NegativeDuration,
  CannotAlterPast,
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
pub struct Goal {
  pub goal_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
}

// changes: took Goal out of TimeUtilityFunction
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
  pub description: String,
  pub time_utility_function: TimeUtilityFunction,
  pub duration_estimate: i64,
  pub scheduled: bool,
  pub start_time: Option<i64>,
  pub duration: Option<i64>,
  pub status: GoalDataStatusKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalDependency {
  pub goal_dependency_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub goal: Goal,
  pub dependent_goal: Goal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastEvent {
  pub past_event_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastEventData {
  pub past_event_data_id: i64,
  pub creation_time: i64,
  pub creator: auth_service_api::response::User,
  pub past_event: PastEvent,
  pub start_time: i64,
  pub duration: i64,
  pub name: String,
  pub description: String,
  pub active: bool,
}
