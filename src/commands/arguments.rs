use crate::config::models::CommandDef;
use crate::config::ParamDef;
use dialoguer::Input;
use regex::Regex;
use std::collections::HashMap;

/**
 Collect arguments for a command, prompting the user for any required
 arguments that are not provided via overrides. Defaults are honored,
 and optional arguments are skipped if not provided.
*/
pub fn substitute_parameters(
    cmd: &CommandDef,
    overrides: Option<&HashMap<String, String>>,
    prompt_for_missing: bool,
) -> HashMap<String, String> {
    let mut collected = overrides.cloned().unwrap_or_default();

    // find placeholders
    let re = Regex::new(r"\{\{(.*?)}}").unwrap();

    for capture in re.captures_iter(&cmd.exec) {
        let placeholder = capture[1].trim();

        // skip already provided
        if collected.contains_key(placeholder) {
            continue;
        }

        // get the parameter definition if present
        let param = cmd
            .params
            .iter()
            .find(|p| p.name == placeholder);

        // fallback ParamDef when missing
        let fallback = ParamDef {
            name: placeholder.to_string(),
            prompt: "".to_string(),
            optional: false,
            default: None,
        };

        let param = param.unwrap_or(&fallback);

        // build prompt
        let mut prompt = if param.prompt.trim().is_empty() {
            format!("Enter value for '{}'", param.name)
        } else {
            param.prompt.clone()
        };

        let mut input = Input::new();
        let mut value = String::new();

        // default value
        if let Some(default) = &param.default {
            input = input.default(default.clone()).show_default(false);
            prompt = format!("{prompt} [default: {default}]");
            value = default.clone();
        }
        // optional field
        else if param.optional {
            input = input.allow_empty(true);
            prompt = format!("{prompt} [optional]");
        }

        // interactive input
        if prompt_for_missing {
            value = input
                .with_prompt(prompt)
                .interact_text()
                .unwrap_or_default();
        }
        if !value.is_empty() || param.optional {
            collected.insert(param.name.clone(), value);
        }
    }

    collected
}