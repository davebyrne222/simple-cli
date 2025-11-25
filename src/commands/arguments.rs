use crate::config::models::CommandDef;
use crate::config::ParamDef;
use dialoguer::Input;
use regex::Regex;
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

    // find all placeholders in the command
    let re = Regex::new(r"\{\{(.*?)}}").unwrap();
    let mut placeholders: Vec<&str> = vec![];
    for (_, [placeholder]) in re.captures_iter(&cmd.exec).map(|c| c.extract()) {
        placeholders.push(placeholder.trim());
    }

    // prompt for missing arguments
    for placeholder in placeholders {

        if collected.contains_key(placeholder) {
            continue;
        }

        // param config provided?
        let default = ParamDef {
            name: placeholder.to_string(),
            prompt: "".to_string(),
            optional: false,
            default: None,
        };
        let param = cmd
            .params
            .iter()
            .find(|p| p.name == placeholder)
            .unwrap_or(&default);

        // build prompt
        let mut input = Input::new();

        let mut prompt = if param.prompt.trim().is_empty() {
            format!("Enter value for '{}'", param.name)
        } else {
            param.prompt.clone()
        };

        if let Some(default) = &param.default {
            input = input.default(default.clone()).show_default(false);
            prompt = format!("{} [default: {}]", prompt, default)
        } else if param.optional {
            input = input.allow_empty(true);
            prompt = format!("{} [optional]", prompt)
        }

        let value: String = input
            .with_prompt(prompt)
            .interact_text()
            .unwrap_or_default();

        if !value.is_empty() {
            collected.insert(param.name.clone(), value);
            continue;
        }
    }

    collected
}
