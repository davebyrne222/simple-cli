use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/** Global configuration loaded from commands.yaml */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub defaults: Option<GlobalDefaults>,
    pub subscriptions: HashMap<String, Subscription>,
    pub categories: Vec<Category>,
}

/** Default values applied across commands */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalDefaults {
    pub subscription: Option<String>,
}

/**
Runtime global context used across modules.
*/
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalContext {
    pub current_subscription: Option<String>,
    pub current_namespace: Option<String>,
    pub current_user: Option<String>,
}

/** Cloud subscription with optional local secrets */
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Subscription {
    pub subscription_id: String,
    // pub resource_group: String,
    pub key_vault: Option<String>,
    pub tenant_id: Option<String>,
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
