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

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NamedEntityKind {
  Date,
  Time,
  Money,
  Url,
  Person,
  Location,
  Hashtag,
  Emoticon,
  Emoji,
  Propn,
  Verb,
}

impl TryFrom<u8> for NamedEntityKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<NamedEntityKind, u8> {
    match val {
      x if x == NamedEntityKind::Date as u8 => Ok(NamedEntityKind::Date),
      x if x == NamedEntityKind::Time as u8 => Ok(NamedEntityKind::Time),
      x if x == NamedEntityKind::Money as u8 => Ok(NamedEntityKind::Money),
      x if x == NamedEntityKind::Url as u8 => Ok(NamedEntityKind::Url),
      x if x == NamedEntityKind::Person as u8 => Ok(NamedEntityKind::Person),
      x if x == NamedEntityKind::Location as u8 => Ok(NamedEntityKind::Location),
      x if x == NamedEntityKind::Hashtag as u8 => Ok(NamedEntityKind::Hashtag),
      x if x == NamedEntityKind::Emoticon as u8 => Ok(NamedEntityKind::Emoticon),
      x if x == NamedEntityKind::Emoji as u8 => Ok(NamedEntityKind::Emoji),
      x if x == NamedEntityKind::Propn as u8 => Ok(NamedEntityKind::Propn),
      x if x == NamedEntityKind::Verb as u8 => Ok(NamedEntityKind::Verb),
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
  pub tags: Vec<String>,
  pub duration_estimate: i64,
  pub time_utility_function_id: i64,
  pub goal_intent_id: Option<i64>,
  pub parent_goal_id: Option<i64>,
  pub time_span: Option<(i64, i64)>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalDataNewProps {
  pub goal_id: i64,
  pub name: String,
  pub tags: Vec<String>,
  pub duration_estimate: i64,
  pub time_utility_function_id: i64,
  pub parent_goal_id: Option<i64>,
  pub status: GoalDataStatusKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalEventNewProps {
  pub goal_id: i64,
  pub start_time: i64,
  pub end_time: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGeneratedCodeNewProps {
  pub source_code: String,
  pub source_lang: String,
  pub wasm_cache: Vec<u8>,
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
pub struct GoalTemplateNew {
  pub name: String,
  pub user_generated_code_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTemplateDataNew {
  pub name: String,
  pub user_generated_code_id: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTemplatePatternNew {
  pub goal_template_id: i64,
  pub pattern: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityNew {
  pub name: String,
  pub kind: NamedEntityKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityDataNew {
  pub named_entity_id: i64,
  pub name: String,
  pub kind: NamedEntityKind,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityPatternNew {
  pub named_entity_id: i64,
  pub pattern: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEventNewProps {
  pub name: String,
  pub start_time: i64,
  pub end_time: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEventDataNewProps {
  pub external_event_id: i64,
  pub name: String,
  pub start_time: i64,
  pub end_time: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentViewProps {
  pub goal_intent_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalIntentDataViewProps {
  pub goal_intent_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_intent_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub responded: Option<bool>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalViewProps {
  pub goal_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_intent_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalDataViewProps {
  pub goal_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub min_duration_estimate: Option<i64>,
  pub max_duration_estimate: Option<i64>,
  pub time_utility_function_id: Option<Vec<i64>>,
  pub parent_goal_id: Option<Vec<i64>>,
  pub status: Option<Vec<GoalDataStatusKind>>,
  pub only_recent: bool,
  pub goal_intent_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalEventViewProps {
  pub goal_event_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGeneratedCodeViewProps {
  pub user_generated_code_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub source_lang: Option<Vec<String>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTemplateViewProps {
  pub goal_template_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTemplateDataViewProps {
  pub goal_template_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_template_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub user_generated_code_id: Option<Vec<i64>>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTemplatePatternViewProps {
  pub goal_template_pattern_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub goal_template_id: Option<Vec<i64>>,
  pub pattern: Option<Vec<String>>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityViewProps {
  pub named_entity_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityDataViewProps {
  pub named_entity_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub named_entity_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub kind: Option<Vec<NamedEntityKind>>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedEntityPatternViewProps {
  pub named_entity_pattern_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub named_entity_id: Option<Vec<i64>>,
  pub pattern: Option<Vec<String>>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeUtilityFunctionViewProps {
  pub time_utility_function_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEventViewProps {
  pub external_event_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalEventDataViewProps {
  pub external_event_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub external_event_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}
