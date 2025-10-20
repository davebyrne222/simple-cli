use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_yaml::Value;

/** Global configuration loaded from commands.yaml */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub defaults: Option<GlobalDefaults>,
    pub groups: HashMap<String, UserParams>,
    pub categories: Vec<Category>,
}

/** Default values applied across commands */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalDefaults {
    pub group: Option<String>,
}

/**
Runtime global context used across modules.
*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalContext {
    pub current_group: Option<String>,
}

/** User parameters with arbitrary key-value pairs (from config.yaml subscriptions).
    All fields are accessible in templates via `config.<key>`. */
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserParams {
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

/** Top-level category grouping commands */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    pub name: String,
    #[serde(default)]
    pub commands: Vec<CommandDef>,
    #[serde(default)]
    pub subcategories: Vec<SubCategory>,
}

/** Nested subcategory grouping commands */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubCategory {
    pub name: String,
    #[serde(default)]
    pub commands: Vec<CommandDef>,
}

/** Command definition loaded from YAML */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandDef {
    pub name: String,
    pub description: String,
    pub exec: String,
    #[serde(default)]
    pub args: Vec<ArgDef>,
    #[serde(default)]
    pub pre_command: Option<String>,
}

/** Argument definition for a command */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArgDef {
    pub name: String,
    pub prompt: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub default: Option<String>,
}
