use std::{collections::HashMap, fs, path::PathBuf, env};
use serde::de::{DeserializeOwned, Error};
use serde_yaml::{Error as YamlError, Value};
use super::models::{ConfigFile, Config};
use log::{debug, warn, error};

pub fn load_config() -> Result<Config, YamlError> {
    let mut config = Config::default();

    debug!("Loading config");
    config.files = get_config_dir(&config.files)?;

    let params_file = config.files
        .get("paramsFile")
        .ok_or_else(|| YamlError::custom("paramsFile missing in config files"))?;

    let commands_file = config.files
        .get("commandsFile")
        .ok_or_else(|| YamlError::custom("commandsFile missing in config files"))?;

    debug!("Loading defaults");
    config.defaults = parse_yaml_file_to_struct(
        &params_file.path,
        Some("defaults".to_string())
    )?;

    debug!("Loading params");
    config.params = parse_yaml_file_to_struct(
        &params_file.path,
        Some("groups".to_string())
    )?;

    debug!("Loading commands");
    config.categories = parse_yaml_file_to_struct(
        &commands_file.path,
        None
    )?;

    Ok(config)
}

/// Finds the config files (tries env var, home dir, cwd)
pub fn get_config_dir(files: &HashMap<String, ConfigFile>) -> Result<HashMap<String, ConfigFile>, YamlError> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    // Envvar
    if let Ok(env_path) = std::env::var("SIMPLE_CLI_DIR") {
        candidates.push(PathBuf::from(env_path));
    } else {
        debug!("SIMPLE_CLI_DIR not set");
    }

    // Home
    if let Some(home_path) = dirs::home_dir() {
        let full_path = home_path.join("SimpleCli");
        if full_path.exists() {
            candidates.push(full_path);
        } else {
            debug!("SimpleCli directory not found at {:?}", full_path);
        }
    }

    // CWD
    match env::current_dir() {
        Ok(cwd) => candidates.push(cwd),
        Err(e) => warn!("Failed to get current working directory: {}", e),
    }

    debug!("Searching for config files in candidate directories: {:?}", candidates);

    for dir in &candidates {
        // Clone the initial state (all ConfigFile structs with empty paths)
        let mut required_files = files.clone();

        let mut all_found = true;

        debug!("Checking directory {:?} for config files", dir);
        for (_name, file) in required_files.iter_mut() {
            let full_path = dir.join(&file.filename);

            if full_path.exists() {
                debug!("Found file '{}' at {:?}", file.filename, full_path);
                file.path = full_path;
            } else {
                debug!("Missing file '{}' at {:?}", file.filename, full_path);
                all_found = false;
                break;
            }
        }

        if all_found {
            debug!("All config files found in {:?}", dir);
            return Ok(required_files);
        }
    }

    error!("Could not locate all config files co-located in any of: {:?}", candidates);
    Err(YamlError::custom(format!(
        "Could not locate all config files co-located in any of: {:?}",
        candidates
    )))
}

/// Reads a YAML file into a struct, optionally extracting a section
fn parse_yaml_file_to_struct<T: DeserializeOwned>(
    path: &PathBuf,
    section: Option<String>,
) -> Result<T, YamlError> {
    debug!("Parsing file {:?} into {}", path, std::any::type_name::<T>());
    let content = fs::read_to_string(path)
        .map_err(|e| YamlError::custom(format!("Failed to read {:?}: {}", path, e)))?;

    let full_yaml: Value = serde_yaml::from_str(&content)
        .map_err(|e| YamlError::custom(format!("Failed to parse {:?}: {}", path, e)))?;

    if let Some(section_name) = section {
        if let Some(section_value) = full_yaml.get(&section_name) {
            serde_yaml::from_value(section_value.clone())
                .map_err(|e| YamlError::custom(format!("Failed to deserialize section '{}' in {:?}: {}", section_name, path, e)))
        } else {
            Err(YamlError::custom(format!("Section '{}' not found in {:?}", section_name, path)))
        }
    } else {
        serde_yaml::from_value(full_yaml)
            .map_err(|e| YamlError::custom(format!("Failed to deserialize file {:?}: {}", path, e)))
    }
}