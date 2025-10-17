/** IO helpers for saving/loading last-used values in the config dir. */
use std::fs;
use std::path::PathBuf;

use dirs::config_dir;

fn value_path(key: &str) -> PathBuf {
    let mut dir = config_dir().unwrap_or_default();
    dir.push("olcs-cli");
    let _ = fs::create_dir_all(&dir);
    dir.push(format!("{}.txt", key));
    dir
}

/** Save a last-used value identified by key. */
#[allow(dead_code)]
pub fn save_last_value(key: &str, value: &str) -> std::io::Result<()> {
    let path = value_path(key);
    fs::write(path, value)
}

/** Load a last-used value identified by key. */
pub fn load_last_value(key: &str) -> Option<String> {
    let path = value_path(key);
    fs::read_to_string(path).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/** Remove all stored app data (e.g., last-used values). */
pub fn clear_saved_data() -> std::io::Result<()> {
    let mut dir = config_dir().unwrap_or_default();
    dir.push("olcs-cli");
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }
    Ok(())
}
