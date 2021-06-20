use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GoalDataStatusKind {
  Succeed,
  Fail,
  Cancel,
  Pending,
}

impl TryFrom<u8> for GoalDataStatusKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<GoalDataStatusKind, u8> {
    match val {
      x if x == GoalDataStatusKind::Succeed as u8 => Ok(GoalDataStatusKind::Succeed),
      x if x == GoalDataStatusKind::Fail as u8 => Ok(GoalDataStatusKind::Fail),
      x if x == GoalDataStatusKind::Cancel as u8 => Ok(GoalDataStatusKind::Cancel),
      x if x == GoalDataStatusKind::Pending as u8 => Ok(GoalDataStatusKind::Pending),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentNewProps {
  pub name: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentDataNewProps {
  pub goal_intent_id: i64,
  pub name: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalNewProps {
  pub name: String,
  pub duration_estimate: i64,
  pub time_utility_function_id: i64,
  pub goal_intent_id: Option<i64>,
  pub parent_goal_id: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalDataNewProps {
  pub goal_id: i64,
  pub name: String,
  pub duration_estimate: i64,
  pub time_utility_function_id: i64,
  pub parent_goal_id: Option<i64>,
  pub status: GoalDataStatusKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunctionNewProps {
  pub start_times: Vec<i64>,
  pub utils: Vec<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEventNewProps {
  pub goal_id: i64,
  pub start_time: i64,
  pub duration: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentViewProps {
  pub goal_intent_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentDataViewProps {
  pub goal_intent_data_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub goal_intent_id: Option<i64>,
  pub name: Option<String>,
  pub partial_name: Option<String>,
  pub responded: Option<bool>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalViewProps {
  pub goal_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub goal_intent_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalDataViewProps {
  pub goal_data_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub goal_id: Option<i64>,
  pub name: Option<String>,
  pub partial_name: Option<String>,
  pub duration_estimate: Option<i64>,
  pub min_duration_estimate: Option<i64>,
  pub max_duration_estimate: Option<i64>,
  pub time_utility_function_id: Option<i64>,
  pub parent_goal_id: Option<i64>,
  pub status: Option<GoalDataStatusKind>,
  pub only_recent: bool,
  pub goal_intent_id: Option<i64>,
  pub scheduled: Option<bool>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskEventViewProps {
  pub task_event_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub goal_id: Option<i64>,
  pub start_time: Option<i64>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub duration: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunctionViewProps {
  pub time_utility_function_id: Option<i64>,
  pub creation_time: Option<i64>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}
