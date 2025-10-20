use std::{collections::HashMap, fs, path::PathBuf, env};
use serde::de::{DeserializeOwned, Error};
use serde_yaml::Error as YamlError;
use crate::config::Category;
use super::models::{GlobalDefaults, UserParams};
use dirs::data_dir;

/** Load default values from config.yaml */
pub fn load_defaults() -> Result<Option<GlobalDefaults>, YamlError> {
    parse_file_to_struct("params.yaml", "defaults")
}

/** Load categories from commands.yaml */
pub fn load_commands() -> Result<Vec<Category>, YamlError>{
    parse_file_to_struct("commands.yaml", "categories")
}

/** Load subscriptions from config.yaml */
pub fn load_groups() -> Result<HashMap<String, UserParams>, YamlError> {
    parse_file_to_struct("params.yaml", "groups")
}

/** Extract and parse yaml section to struct */
fn parse_file_to_struct<T: DeserializeOwned>(file_path: &str, section: &str) -> Result<T, YamlError> {
    use serde_yaml::Value;

    // Candidate locations (prefer user data dir, then next to the executable, then current working directory)
    let mut candidates: Vec<PathBuf> = Vec::new();

    // 1) User data dir, e.g. ~/.local/share/olcs-cli/<file_path>
    if let Some(mut dd) = data_dir() {
        dd.push("olcs-cli");
        candidates.push(dd.join(file_path));
    }

    // 2) Directory of the current executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join(file_path));
        }
    }

    // 3) Current working directory
    candidates.push(PathBuf::from(file_path));

    // Try each candidate path in order
    let mut last_err: Option<YamlError> = None;
    let (used_path, content) = match candidates.iter().find_map(|p| {
        match fs::read_to_string(p) {
            Ok(c) => Some((p.clone(), c)),
            Err(e) => {
                last_err = Some(YamlError::custom(format!("Failed to read {:?}: {}", p, e)));
                None
            }
        }
    }) {
        Some(ok) => ok,
        None => {
            let tried = candidates
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(YamlError::custom(format!("Could not read {} from any of: {}", file_path, tried)));
        }
    };

    let full_yaml: Value = serde_yaml::from_str(&content)
        .map_err(|e| YamlError::custom(format!("Failed to parse {:?}: {}", used_path, e)))?;

    // Extract the specific section
    if let Some(section_value) = full_yaml.get(section) {
        serde_yaml::from_value(section_value.clone())
    } else {
        serde_yaml::from_str("null")
    }
}

/** Resolve the effective path to a config/params file using the same search order. */
pub fn resolve_config_path(file_path: &str) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Some(mut dd) = data_dir() {
        dd.push("olcs-cli");
        candidates.push(dd.join(file_path));
    }

    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join(file_path));
        }
    }

    candidates.push(PathBuf::from(file_path));

    candidates.into_iter().find(|p| p.exists())
}
