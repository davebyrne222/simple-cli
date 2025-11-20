use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use serde_yaml::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    pub filename: String,
    pub path: PathBuf,
}

/** Global configuration loaded from config files */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub defaults: Option<GlobalDefaults>,
    pub params: HashMap<String, UserParams>,
    pub categories: Vec<Category>,
    pub files: HashMap<String, ConfigFile>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: None,
            params: HashMap::new(),
            categories: Vec::new(),
            files: HashMap::from([
                ("paramsFile".to_string(), ConfigFile { filename: "scli.params.yaml".to_string(), path: PathBuf::new() }),
                ("commandsFile".to_string(), ConfigFile { filename: "scli.commands.yaml".to_string(), path: PathBuf::new() })
            ]),
        }
    }
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
    pub category: String,
    #[serde(default)]
    pub description: String,
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
    pub description: String,
    #[serde(default)]
    pub commands: Vec<CommandDef>,
}

/** Command definition loaded from YAML */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub exec: String,
    #[serde(default)]
    pub params: Vec<ParamDef>,
    #[serde(default)]
    pub pre_command: Option<String>,
}

/** Argument definition for a command */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParamDef {
    pub name: String,
    pub prompt: String,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub default: Option<String>,
}
