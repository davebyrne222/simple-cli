# simple-cli — YAML‑driven Command Runner
![version 0.1.0](https://img.shields.io/badge/status-wip-yellow)
![Version](https://img.shields.io/github/v/tag/davebyrne222/simple-cli?label=latest%20version&color=blue)

`simple-cli` is a lightweight, YAML-driven CLI that lets you define repeatable command workflows in a single `commands.yaml` and
run them either directly or via an interactive menu. It’s designed to streamline everyday operations tasks such as
invoking cloud CLIs, Kubernetes commands, or custom scripts—consistently and safely.

Key capabilities:

- Human-friendly command catalog in YAML: categories, subcategories, and commands.
- Interactive mode with a navigable menu and prompts for arguments.
- Powerful templating for command substitution and composition.
- Optional pre-commands to chain or prepare context.
- Works with your existing tooling (e.g., `az`, `kubectl`, `jq`) by orchestrating shell commands you already use.

---

## Quick Start

1) Ensure you have:

- A working shell environment with any tools your commands will call (e.g., Azure CLI `az`, `kubectl`, `jq`).
- A `commands.yaml` file in your working directory (see examples below).

1) Run in interactive mode:

- `olcs -i`
- Browse categories, pick a command, and fill in prompts as needed.

1) Run a command directly:

- `olcs demo.basic`
- With arguments: `olcs demo.args --arg name=Dave` (if supported by the command)

Tip: If an argument value includes spaces or special characters, wrap it in quotes.

---

## The commands.yaml file

Your command catalog is defined in a single YAML file. A typical structure includes:

- `defaults`: Optional global defaults that can be used in templates.
- `categories` → `subcategories` → `commands`: Logical organization of commands.
- Command fields:
    - `name`: The CLI name for the command.
    - `description`: What the command does (shown in interactive mode).
    - `exec`: The shell command to run. Supports template replacement for args/config.
    - `args`: Optional list of arguments. Each arg can have `name`, `prompt`, `default`, and `optional`.
    - `pre_command`: Optionally run another defined command before this one (e.g., to list/select resources).

Templating:

- Use `{{ ... }}` in `exec` to substitute values from args or config.
- For interactive selection of a parameter, use the interactive parameter filter:
    - Example: `{{ "namespace" | i_param }}` will prompt the user for a namespace in interactive mode.

Examples you can try (from a typical catalog):

- Demo:
    - `olcs demo.basic` → prints a greeting.
    - `olcs demo.args --arg name=Dave` → passes an argument.
    - `olcs -i` → find “Demo > interactive” to see interactive prompting.
    - `olcs demo.config` → shows using a value from configuration, e.g. `{{ config.subscription_id }}`.
    - `olcs demo.namespace --namespace="olcs"` → demonstrates selecting/overriding a parameter.
- Azure:
    - `olcs az.subscription.login`
    - `olcs az.subscription.list-subscriptions`
    - `olcs az.key-vault.list-secrets`
- Kubernetes:
    - `olcs kubernetes.config.view-config`
    - `olcs kubernetes.context.switch-context`
    - `olcs kubernetes.pods.list-pods --arg namespace=olcs`
    - `olcs kubernetes.pods.pod-logs` (will prompt you to pick a pod)
    - `olcs kubernetes.deployments.scale-deployment`
    - `olcs kubernetes.secrets.list-secrets`
- Config management:
    - `olcs config.pull-latest-config` → fetch a fresh `commands.yaml` from Azure DevOps.

Note: The concrete command paths in your CLI depend on how categories/subcategories are named in your `commands.yaml` (
e.g., `Demo.basic` can be invoked as `olcs demo.basic`).

---

## Installation

Build from source:

1) Install Rust (stable) and Cargo.
2) From the project root:
    - `cargo build --release`
3) The compiled binary will be in `target/release/olcs`. Put it on your PATH or run it directly:
    - `./target/release/olcs -i`

Alternatively, if you package or distribute binaries in your environment, place the binary on your PATH and run `olcs`.

### Tool dependencies
Your `exec` entries can call any tool available in your shell. For common examples:

- Azure: `az login`, `az account set`, `az keyvault ...`
- Kubernetes: `kubectl`, including contexts, namespaces, pods, logs, scaling, and secrets
- JSON viewing: `jq` (e.g., when parsing logs)

Ensure these tools are installed and on your PATH when running commands that reference them.


### Customizing the CLI name
By default, the cli is called using 'olcs' however, you can call the CLI whatever you want by setting the binary’s name
in the `Cargo.toml` file.

---

## Updating Your Catalog

You can keep your catalog fresh by pulling a new `commands.yaml` from your source of truth. If your project uses a
DevOps repo to store the YAML, a helper command like `config.pull-latest-config` can automate fetching and replacing the
local file.

---

## Tips and Troubleshooting

- Quoting matters:
    - When passing values with spaces or special characters, wrap them in quotes: `--arg namespace="my ns"`.
- YAML hygiene:
    - Keep indentation consistent. Comments or stray tabs can break parsing.
- Command not found:
    - If a command in `exec` fails with “not found,” install the tool or add it to your PATH.
- Test directly then codify:
    - First run a command in your shell, then copy it into `exec` and parameterize with `{{ ... }}`.
- Start interactive:
    - If you’re unsure about argument names or defaults, try `olcs -i`.

---

## Contributing

- Propose improvements to `commands.yaml` structure, naming, and descriptions to keep the catalog consistent and
  discoverable.
- Prefer small, composable commands connected with `pre_command` over large, opaque scripts.
- Add clear prompts and defaults to improve interactive UX.

---
