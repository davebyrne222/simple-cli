/** Argument collection and prompting. */
use std::collections::HashMap;
use crate::config::models::CommandDef;
use dialoguer::Input;

/**
 Collect arguments for a command, prompting the user for any required
 arguments that are not provided via overrides. Defaults are honored,
 and optional arguments are skipped if not provided.
*/
pub fn collect_arguments(cmd: &CommandDef, overrides: &HashMap<String, String>) -> HashMap<String, String> {
    let mut collected = overrides.clone();

    for arg in &cmd.args {
        if collected.contains_key(&arg.name) {
            continue;
        }

        if let Some(default) = &arg.default {
            collected.insert(arg.name.clone(), default.clone());
            continue;
        }

        if arg.optional {
            // Optional and not provided; skip
            continue;
        }

        // Prompt for required argument
        let prompt = if arg.prompt.trim().is_empty() {
            format!("Enter value for {}", arg.name)
        } else {
            arg.prompt.clone()
        };

        let value: String = Input::new()
            .with_prompt(prompt)
            .interact_text()
            .unwrap_or_default();

        if !value.is_empty() {
            collected.insert(arg.name.clone(), value);
        }
    }

    collected
}
