use dirs::config_dir;
use std::fs;
use std::path::PathBuf;

use super::models::{Config, GlobalContext};
use crate::utils::io::{load_last_value, save_last_value};

/** Create a global context using defaults and last-used values. */
pub fn create_context(cfg: &Config) -> GlobalContext {
    let mut ctx = GlobalContext::default();

    if let Some(def) = &cfg.defaults {
        ctx.current_group = def.group.clone();
    }

    // Load last used values (if present)
    if let Some(val) = load_last_value("group") { ctx.current_group = Some(val); }
    if let Some(val) = load_last_value("namespace") { ctx.current_namespace = Some(val); }
    if let Some(val) = load_last_value("user") { ctx.current_user = Some(val); }

    ctx
}

/** Persist last-used value by key inside the config data directory. */
#[allow(dead_code)]
pub fn set_last_used(key: &str, value: &str) {
    let _ = save_last_value(key, value);
}

/** Convenience to get the config data dir used by this app. */
#[allow(dead_code)]
pub fn config_data_dir() -> PathBuf {
    let mut dir = config_dir().unwrap_or_default();
    dir.push("olcs-cli");
    fs::create_dir_all(&dir).ok();
    dir
}
