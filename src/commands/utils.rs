/** Miscellaneous helpers for command discovery and normalization. */
use crate::config::models::{Category, CommandDef};


/** Normalize a name for comparisons (lowercase, trim). */
pub fn normalize_name(s: &str) -> String {
    s.to_lowercase().replace(" ", "")
}

/** Find a command by name across categories and subcategories. */
pub fn find_command<'a>(categories: &'a [Category], name: &str) -> Option<&'a CommandDef> {
    let search_cmd_name = normalize_name(name);
    for cat in categories {
        for cmd in &cat.commands {
            let cmd_name = normalize_name(&format!("{}.{}", &cat.name, &cmd.name));
            if cmd_name == search_cmd_name {
                return Some(cmd);
            }
        }
        for sub in &cat.subcategories {
            for cmd in &sub.commands {
                let cmd_name = normalize_name(&format!("{}.{}.{}", &cat.name, &sub.name, &cmd.name));
                if cmd_name == search_cmd_name {
                    return Some(cmd);
                }
            }
        }
    }
    None
}
