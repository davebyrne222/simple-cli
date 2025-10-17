use std::collections::HashMap;
use colored::Colorize;
use crate::config::{Config, CommandDef, GlobalContext, Subscription};
use crate::commands::render::render_cmd;
use crate::utils::shell::execute_shell_command;

pub fn run_command(
    cmd: &CommandDef,
    cfg: &Config,
    ctx: &mut GlobalContext,
    args: &HashMap<String, String>,
) -> Result<(), String> {

    let sub_name = ctx.current_subscription.as_ref()
        .ok_or_else(|| "No subscription selected".to_string())?;

    let current_config: &Subscription = cfg.subscriptions.get(sub_name)
        .ok_or_else(|| format!("Subscription '{}' not found", sub_name))?;

    let rendered = render_cmd(cmd, current_config, ctx, args)
        .map_err(|e| format!("Failed to render: {}", format_error_chain(&e)))?;

    // Colored, minimal shell-like prefix: `$ command`
    println!("{} {}", "$".blue().bold(), rendered.as_str().bright_black());

    execute_shell_command(&rendered)
        .map_err(|e| format!("Failed to execute: {}", e))?;

    Ok(())
}

fn format_error_chain(err: &dyn std::error::Error) -> String {
    let mut parts: Vec<String> = Vec::new();
    parts.push(err.to_string());
    let mut cur = err.source();
    while let Some(next) = cur {
        parts.push(next.to_string());
        cur = next.source();
    }
    parts.join(":\n ")
}