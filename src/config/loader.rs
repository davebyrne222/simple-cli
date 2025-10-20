use std::{collections::HashMap, fs, path::PathBuf, env};
use serde::de::{DeserializeOwned, Error};
use serde_yaml::{Error as YamlError, Value};
use crate::config::{Category, CommandDef, SubCategory};
use super::models::{GlobalDefaults, UserParams};
use dirs::data_dir;

/** Load default values from config.yaml */
pub fn load_defaults() -> Result<Option<GlobalDefaults>, YamlError> {
    // parse_file_to_struct("params.yaml", Some(String::from("defaults")))
    let commands_path = get_candidate_file_path("params.yaml")?;
    parse_yaml_file_to_struct(&commands_path, Some(String::from("defaults")))
}

/** Load subscriptions from config.yaml */
pub fn load_groups() -> Result<HashMap<String, UserParams>, YamlError> {
    // parse_file_to_struct("params.yaml", Some(String::from("groups")))
    let commands_path = get_candidate_file_path("params.yaml")?;
    parse_yaml_file_to_struct(&commands_path, Some(String::from("groups")))
}

/** Load categories from commands.yaml */
pub fn load_commands() -> Result<Vec<Category>, YamlError> {
    // Load the base (required) commands.yaml
    let commands_path = get_candidate_file_path("commands.yaml")?;
    let mut commands: Vec<Category> = parse_yaml_file_to_struct(&commands_path, None)?;

    // Try loading the local overrides (optional)
    if let Ok(local_path) = get_candidate_file_path("commands.local.yaml") {
        if let Ok(local_commands) = parse_yaml_file_to_struct::<Vec<Category>>(&local_path, None) {
            merge_categories(&mut commands, local_commands);
        }
    }

    Ok(commands)
}

/// Finds the best candidate file path (tries data_dir, exe dir, cwd)
fn get_candidate_file_path(file_name: &str) -> Result<PathBuf, YamlError> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    // 1. User data dir, e.g. ~/.local/share/olcs-cli/<file_name>
    if let Some(mut data_dir) = dirs::data_dir() {
        data_dir.push("olcs-cli");
        candidates.push(data_dir.join(file_name));
    }

    // 2. Directory of current executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            candidates.push(exe_dir.join(file_name));
        }
    }

    // 3. Current working directory
    candidates.push(PathBuf::from(file_name));

    for path in &candidates {
        if path.exists() {
            return Ok(path.clone());
        }
    }

    Err(YamlError::custom(format!(
        "Could not locate file {} in any of: {:?}",
        file_name, candidates
    )))
}

/// Reads a YAML file into a struct, optionally extracting a section
fn parse_yaml_file_to_struct<T: DeserializeOwned>(
    path: &PathBuf,
    section: Option<String>,
) -> Result<T, YamlError> {
    let content = fs::read_to_string(path)
        .map_err(|e| YamlError::custom(format!("Failed to read {:?}: {}", path, e)))?;

    let full_yaml: Value = serde_yaml::from_str(&content)
        .map_err(|e| YamlError::custom(format!("Failed to parse {:?}: {}", path, e)))?;

    if let Some(section_name) = section {
        if let Some(section_value) = full_yaml.get(section_name) {
            serde_yaml::from_value(section_value.clone())
        } else {
            serde_yaml::from_str("null")
        }
    } else {
        serde_yaml::from_value(full_yaml)
    }
}

/// Merge local categories and subcategories recursively.
/// Local data always overrides base data when names match.
fn merge_categories(base: &mut Vec<Category>, local: Vec<Category>) {
    for local_cat in local {
        if let Some(existing_cat) = base.iter_mut().find(|c| c.category == local_cat.category) {
            // Override category description if provided
            if !local_cat.description.is_empty() {
                existing_cat.description = local_cat.description.clone();
            }

            // Merge commands
            merge_commands_list(&mut existing_cat.commands, local_cat.commands);

            // Merge subcategories recursively
            merge_subcategories(&mut existing_cat.subcategories, local_cat.subcategories);
        } else {
            // Entirely new category
            base.push(local_cat);
        }
    }
}

/// Merge subcategories by name, giving precedence to local definitions.
fn merge_subcategories(base: &mut Vec<SubCategory>, local: Vec<SubCategory>) {
    for local_sub in local {
        if let Some(existing_sub) = base.iter_mut().find(|s| s.name == local_sub.name) {
            // Override subcategory description if provided
            if !local_sub.description.is_empty() {
                existing_sub.description = local_sub.description.clone();
            }

            // Merge commands
            merge_commands_list(&mut existing_sub.commands, local_sub.commands);
        } else {
            // Entirely new subcategory
            base.push(local_sub);
        }
    }
}

/// Merge commands, local takes precedence by name.
fn merge_commands_list(base: &mut Vec<CommandDef>, local: Vec<CommandDef>) {
    for local_cmd in local {
        if let Some(existing_cmd) = base.iter_mut().find(|c| c.name == local_cmd.name) {
            *existing_cmd = local_cmd;
        } else {
            base.push(local_cmd);
        }
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

    for p in candidates {
        if p.exists() {
            // Prefer absolute (canonical) path; fall back to the found path on error.
            if let Ok(abs) = fs::canonicalize(&p) {
                return Some(abs);
            } else {
                return Some(p);
            }
        }
    }
    None
}
