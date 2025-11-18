mod cli;
mod commands;
mod config;
mod interactive;
mod utils;

use crate::commands::cli_utils::{handle_switch_subscription, list_commands};
use clap::Parser;
use cli::Cli;
use commands::{find_command, run_command};
use config::{
    create_context, load_config, Config,
};
use interactive::run_interactive;
use crate::utils::io::clear_saved_data;
use serde_yaml;
use std::collections::HashMap;
use std::path::PathBuf;
use log::warn;

/// Entry point: init: load config and initialize context
fn main() {
    // init logger
    env_logger::init();

    // loqd values from files
    let config = load_config().expect("Failed to load config");

    // Initialize global context
    let mut global_ctx = create_context(&config);

    // Handle commands
    handle_args(&config, &mut global_ctx);
}

/// parse CLI and dispatch to interactive or directly execute.
fn handle_args(config: &Config, mut global_ctx: &mut config::GlobalContext) {
    //parse CLI
    let cli = Cli::parse();

    // Clear stored data and exit
    if cli.clear_stored {
        match clear_saved_data() {
            Ok(_) => println!("Cleared stored data."),
            Err(e) => eprintln!("Failed to clear stored data: {}", e),
        }
        return;
    }

    // Show config and exit
    if cli.show_active_params {
        let active_group = global_ctx.current_group.as_ref().unwrap();

        let path = match config.files.get("paramsFile") {
            Some(file) => file.path.clone(),
            None => {
                warn!("paramsFile is missing in config");
                PathBuf::from("scli.params.yaml")
            }
        };

        println!("Params file: {}", path.display());

        match serde_yaml::to_string(&config.params.get(active_group)) {
            Ok(subs_yaml) => {
                println!("Active group: {}", active_group);
                println!("Params:\n  {}", subs_yaml.replace("\n", "\n  "));
            }
            Err(e) => {
                eprintln!("Failed to serialize groups to YAML: {}", e);
            }
        }
        return;
    }

    // Switch subscription
    if cli.switch_param_group {
        handle_switch_subscription(config, &mut global_ctx);
        return;
    }

    // List commands
    if cli.list_cmds {
        list_commands(&config);
        return;
    }

    // Build argument overrides from cli.args
    let mut arg_overrides: HashMap<String, String> = HashMap::new();
    for arg in &cli.args {
        if let Some((k, v)) = arg.split_once('=') {
            arg_overrides.insert(k.to_string(), v.to_string());
        }
    }

    // Run interactive mode
    if cli.interactive {
        run_interactive(&config, &mut global_ctx, &arg_overrides);
        return;
    }

    // Run direct command
    if let Some(cmd_name) = cli.command {
        match find_command(&config.categories, &cmd_name) {
            Some(cmd) => {
                if let Err(e) = run_command(cmd, config, &mut global_ctx, &arg_overrides) {
                    eprintln!("Failed to execute command: {}", e);
                }
            }
            None => eprintln!("Unknown command: {}", cmd_name),
        }
        return;
    }

    println!("No command given. Try --interactive, --list, or specify a command.");
}
