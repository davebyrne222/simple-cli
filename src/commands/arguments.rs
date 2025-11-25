use crate::config::models::CommandDef;
use dialoguer::Input;
/** Argument collection and prompting. */
use std::collections::HashMap;

/**
 Collect arguments for a command, prompting the user for any required
 arguments that are not provided via overrides. Defaults are honored,
 and optional arguments are skipped if not provided.
*/
pub fn collect_arguments(
    cmd: &CommandDef,
    overrides: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut collected = overrides.clone();

    for arg in &cmd.params {
        if collected.contains_key(&arg.name) {
            continue;
        }

        // Prompt for the required argument
        let mut prompt = if arg.prompt.trim().is_empty() {
            format!("Enter value for {}", arg.name)
        } else {
            arg.prompt.clone()
        };

        let mut input = Input::new();

        if let Some(default) = &arg.default {
            input = input.default(default.clone()).show_default(false);
            prompt = format!("{} [default: {}]", prompt, default)
        } else if arg.optional {
            input = input.allow_empty(true);
            prompt = format!("{} [optional]", prompt)
        }

        let value: String = input
            .with_prompt(prompt)
            .interact_text()
            .unwrap_or_default();

        if !value.is_empty() {
            collected.insert(arg.name.clone(), value);
            continue;
        }
    }

    collected
}
