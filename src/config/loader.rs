use std::{collections::HashMap, fs, path::{Path, PathBuf}, env};
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use super::models::{ConfigFile, Config};
use log::{debug, info, warn, error};
use thiserror::Error;

/// Error type for configuration loading
#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("Requested section '{0}' is missing in {1:?}")]
    MissingSection(String, PathBuf),

    #[error("Config file key missing: {0}")]
    MissingConfigKey(String),

    #[error("Could not find all config files in any candidate directory:\n{0}")]
    MissingConfigFiles(String),
}

/// Top-level config loader
pub fn load_config() -> Result<Config, ConfigLoadError> {
    let mut config = Config::default();

    debug!("Starting config load");

    // Locate directory containing all required ConfigFiles
    config.files = get_config_dir(&config.files)?;

    // Get file paths
    let params_file = config.files
        .get("paramsFile")
        .ok_or_else(|| ConfigLoadError::MissingConfigKey("paramsFile".to_string()))?;
    let commands_file = config.files
        .get("commandsFile")
        .ok_or_else(|| ConfigLoadError::MissingConfigKey("commandsFile".to_string()))?;

    // Read the params file
    debug!("Reading params file at {:?}", params_file.path);
    let params_yaml = read_yaml_file(&params_file.path)?;

    debug!("Loading defaults from params file");
    config.defaults = parse_section_from_value(&params_yaml, Some("defaults"), &params_file.path)?;

    debug!("Loading params/groups from params file");
    config.params = parse_section_from_value(&params_yaml, Some("groups"), &params_file.path)?;

    // Read commands file
    debug!("Reading commands file at {:?}", commands_file.path);
    let commands_yaml = read_yaml_file(&commands_file.path)?;

    debug!("Loading commands/categories from commands file");
    config.categories = parse_section_from_value(&commands_yaml, None, &commands_file.path)?;

    info!("Configuration loaded successfully");
    Ok(config)
}

/// Given a map of expected files (key -> ConfigFile with `filename` set),
/// find a directory that contains *all* those files co-located.
///
/// Candidate search order: $SIMPLE_CLI_DIR, $HOME/SimpleCli, current working dir.
pub fn get_config_dir(files: &HashMap<String, ConfigFile>) -> Result<HashMap<String, ConfigFile>, ConfigLoadError> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    // Envvar
    if let Ok(env_path) = env::var("SIMPLE_CLI_DIR") {
        let p = PathBuf::from(env_path);
        debug!("Candidate from SIMPLE_CLI_DIR: {:?}", p);
        candidates.push(p);
    } else {
        debug!("SIMPLE_CLI_DIR not set");
    }

    // Home
    if let Some(home) = dirs::home_dir() {
        let p = home.join("SimpleCli");
        debug!("Checking home candidate: {:?}", p);
        if p.exists() {
            candidates.push(p);
        } else {
            debug!("Home candidate missing: {:?}", p);
        }
    }

    // CWD
    match env::current_dir() {
        Ok(cwd) => {
            debug!("Using current dir candidate: {:?}", cwd);
            candidates.push(cwd);
        }
        Err(e) => warn!("Failed to get current working directory: {}", e),
    }

    if candidates.is_empty() {
        return Err(ConfigLoadError::MissingConfigFiles(
            "No candidate directories available".to_string()
        ));
    }

    // For diagnostics: which files are missing per directory
    let mut diagnostics: Vec<(PathBuf, Vec<String>)> = Vec::new();

    // Try each candidate path
    for dir in &candidates {
        let mut cloned_map = files.clone();
        let mut missing: Vec<String> = Vec::new();

        debug!("Checking candidate dir {:?}", dir);

        for (_key, cfg_file) in cloned_map.iter_mut() {
            let path = dir.join(&cfg_file.filename);

            if path.exists() {
                debug!("  {:?}: found", path);
                cfg_file.path = path;
            } else {
                debug!("  {:?}: missing", path);
                missing.push(cfg_file.filename.clone());
            }
        }

        if missing.is_empty() {
            debug!("All required files found in {:?}", dir);
            return Ok(cloned_map);
        } else {
            debug!("Candidate {:?} is missing files: {:?}", dir, missing);
            diagnostics.push((dir.clone(), missing));
        }
    }

    // Message describing missing files per candidate
    let mut msg = String::from("Missing files per candidate directory:\n");
    for (dir, missing) in &diagnostics {
        msg.push_str(&format!("- {:?} missing: {:?}\n", dir, missing));
    }
    msg.push_str("Searched candidates: SIMPLE_CLI_DIR, $HOME/SimpleCli, current working directory.");

    error!("{}", msg);
    Err(ConfigLoadError::MissingConfigFiles(msg))
}

/// Read a YAML file return a serde_yaml::Value
fn read_yaml_file(path: &Path) -> Result<Value, ConfigLoadError> {
    debug!("Reading YAML file {:?}", path);
    let content = fs::read_to_string(path)?; // Io error -> ConfigLoadError::Io via From
    let value: Value = serde_yaml::from_str(&content)?; // serde_yaml::Error -> ConfigLoadError::Yaml
    Ok(value)
}

/// Extract a section (or deserialize the whole file if `section` is None)
/// from a serde_yaml::Value and turn it into T.
fn parse_section_from_value<T: DeserializeOwned>(
    full_yaml: &Value,
    section: Option<&str>,
    path: &Path,
) -> Result<T, ConfigLoadError> {
    match section {
        Some(name) => {
            match full_yaml.get(name) {
                Some(value) => {
                    let v = value.clone();
                    let t = serde_yaml::from_value(v)?;
                    Ok(t)
                }
                None => Err(ConfigLoadError::MissingSection(name.to_string(), path.to_path_buf())),
            }
        }
        None => {
            let v = full_yaml.clone();
            let t = serde_yaml::from_value(v)?;
            Ok(t)
        }
    }
}