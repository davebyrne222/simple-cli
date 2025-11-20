<div align="center">

<img src="docs/images/Logo2.png" width="400" alt="SimpleCli Logo"/>

<h1> SimpleCli </h1>

<h2>A configurable, YAML-driven, command runner CLI</h2>

Define repeatable command workflows with argument substitution and run them in your terminal.

![Status](https://img.shields.io/badge/status-wip-yellow)
![Version](https://img.shields.io/github/v/tag/davebyrne222/SimpleCli?label=latest%20version&color=blue)

</div>

CLIs are powerful, but remembering commands, flags, and argument values is not.

_SimpleCli_ lets you save and reuse them in a straightforward YAML file, keeping your workflows consistent and repeatable.

Key capabilities:
- Command catalogues defined in YAML
- Interactive mode with a simple menu and argument prompts
- Flexible templating for argument substitution and command composition
- Works with your existing tools (az, kubectl, jq, etc.) by orchestrating the shell commands you already use

---

# Quick Start

## Basic Usage

Ensure you have:

- A working shell environment with any tools your commands will call (e.g., Azure CLI `az`, `kubectl`, `jq`).
- A `scli.commands.yaml` file in your working directory with commands listed as follows (see below for more examples):

```yaml
- name: basic
  description: Create a command and call it from the CLI e.g. `olcs demo.basic`
  exec: echo "Hello, world!"
```

To Run a command directly simply use its full path from the `scli.commands.yaml` file, for example, `scli basic`.

Tip: If an argument value includes spaces or special characters, wrap it in quotes.

## Interactive Mode

To run in interactive mode, use the `-i` flag: `scli -i`. This will present an interactive menu of categories,
subcategories, and commands which you can navigate through and select a command. If the command requires user input, it
will prompt for a value.

<div><p style="color:red">Insert image of an interactive menu</p></div>

---

# Commands and Parameter Files

_SimpleCli_ utilises two files:

- `scli.commands.yaml` command definition file
- `scli.params.yaml` parameter values for command argument substitution. **Optional**

These files can be stored in a number of locations, as listed in order of search preference:

1. Directory specified by the `SIMPLE_CLI_DIR` environment variable.
2. SimpleCli directory in the user home directory i.e. `$HOME/SimpleCli`
3. Current working directory.

## The `scli.commands.yaml` file

Your catalogue of commands is defined in this YAML file. The basic structure is to list commands as follows:

```yaml
- name: basic
  description: Create a command and call it from the CLI e.g. `olcs demo.basic`
  exec: echo "Hello, world!"
```

Where the available command fields are:

- `name`: The CLI name for the command.
- `description`: What the command does (shown in interactive mode).
- `exec`: The shell command to run.

For better organisation, categories and subcategories can also be used:

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Basic
      description: Create a command and call it from the CLI e.g. `olcs demo.basic`
      exec: echo "Hello, world!"
  subcategories:
    - name: Subcategory
      description: A subcategory of commands
      commands:
        - name: params
          description: An example of a subcategory command
          exec: echo "Hello from the subcategory!"
```

These commands can be invoked from the CLI as `scli demo.basic` and `scli demo.subcategory.params`, respectively.

The example `scli.commands.yaml` file included with this project and the [Advanced Usage](#advanced-usage) section
provide further examples of the usage including argument substitution and composition.

## The `scli.params.yaml` file

TBD

---

# Advanced Usage

## Argument Substitution by Command Line

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Args
      description: An example of argument substitution
      exec: echo "Hello, my name is {{ name }}
```

If a command requires an argument override, it can be passed as argument to the command line by using the `--arg` flag.
For the example above: `scli demo.args --arg name=Dave`

<div><p style="color:red">What happens when arg is not provided? Can set a generic prompt: "Provide value for 'name'"?</p></div>

## Argument Substitution by Prompting

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Args
      description: An example of argument substitution
      exec: echo "Hello, my name is {{ name }}
      args:
        - name: name
          prompt: Enter your name
```

Now, when running the command, the CLI will prompt for the value of `name`. This will be prompted in both interactive
and non-interactive mode.

<div><p style="color:red">Check this!</p></div>

- `args`: Optional list of arguments. Each arg can have `name`, `prompt`, `default`, and `optional`.

## Argument Substitution from Params File

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Args
      description: An example of argument substitution
      exec: echo "Hello, my name is {{ params.name }}
```

Now, running this command will cause the value to be substituted from the `scli.params.yaml` file, specifically the
active group's `name` parameter.

Changing the active group (`scli -s`) in the params file will cause the command to run with the new value.

## Optional Arguments

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Args
      description: An example of argument substitution
      exec: >
        echo "Hello, my name is {{ name }}
        {% if country %} and I live in {{ country }}{% endif %}
      args:
        - name: name
          prompt: Enter your name
        - name: country
          prompt: Enter your country of origin (optional)
          optional: true
```

<div><p style="color:red">Not working</p></div>

## Default Values

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: Args
      description: An example of argument substitution
      exec: echo "Hello, my name is {{ name }}
      args:
        - name: name
          prompt: Enter your name
          default: Dave
```

To provide a default value, specify it in the `default` field. Now if the user does not provide a value for
`name`, it will be substituted with the default value.

<div><p style="color:red">Not prompting for name</p></div>

## Pre-Commands

```yaml
- category: Demo
  description: A collection of demo commands showing how to use the CLI
  commands:
    - name: ListFlavours
      description: List available flavours
      exec: printf "Vanilla, Strawberry, Chocolate\n" | tr "," "\n"
    - name: SelectFlavour
      description: Ask the user to select a flavour
      pre_command: demo.listflavours
      exec: echo "A {{ flavour }} milkshake, coming right up!"
      args:
        - name: flavour
          prompt: Which flavour do you want?
```

Optionally, run another defined command before this one. This, for example, could be to provide a reminder of possible
values.

---

# Installation

Build from source:

1. Install Rust (stable) and Cargo.
2. From the project root:
    - `cargo build --release`
3. The compiled binary will be in `target/release/scli`. Put it on your PATH or run it directly:
    - `./target/release/scli -i`

## Dependencies

Your `exec` entries can call any tool available in your shell. For common examples:

- Azure: `az login`, `az account set`, `az keyvault ...`
- Kubernetes: `kubectl`, including contexts, namespaces, pods, logs, scaling, and secrets
- JSON viewing: `jq` (e.g., when parsing logs)

Ensure these tools are installed and on your PATH when running commands that reference them.

## Customising the CLI name

By default, the cli binary is named `scli` however, you can call the CLI whatever you want by setting the binary’s name
in the `Cargo.toml` file and rebuilding:

```toml
[[bin]]
name = "scli"
```

Or alternatively, by creating a wrapper function in your shells `~/.bashrc`, `~/.zshrc`, or `~/.profile`:

```bash
mycli() {
    scli "$@"
}
```

---

# Tips and Troubleshooting

- Quoting matters:
    - When passing values with spaces or special characters, wrap them in quotes: `--arg namespace="my ns"`.
- YAML hygiene:
    - Keep indentation consistent. Comments or stray tabs can break parsing.
- Command not found:
    - If a command in `exec` fails with “not found”, install the tool or add it to your PATH.
- Test directly, then codify:
    - First run a command in your shell, then copy it into `exec` and parameterise with `{{ ... }}`.
- Start interactive:
    - If you’re unsure about argument names or defaults, try `scli -i`.

---

# Some _Hidden_ Features

_SimpleCli_ started out as a personal project for use in DevOps with Azure. Here are some features that I added
that may be useful to others:

## Kubectl Namespace Selection

For interactive selection of the namespace, including setting a specific namespace with `-n` or specifying all
namespaces
via `--all-namespaces`, use the interactive parameter filter `{{ "namespace" | i_param }}`.
This will list all namespaces and will prompt the user to select one an option. For example:

```yaml
exec: kubectl get pods {{ "namespace" | i_param }} # note that the namespace flag '-n' is omitted
```

---

# Contributing

- Propose improvements to `commands.yaml` structure, naming, and descriptions to keep the catalogue consistent and
  discoverable.
- Prefer small, composable commands connected with `pre_command` over large, opaque scripts.
- Add clear prompts and defaults to improve interactive UX.

# Future Plans:

- Add support for argument value prompt options i.e. a list of options to choose from for an argument.
    - Support command execution or script execution for argument values.
- Retrieve values from local secure storage, e.g. `{{"mysecret" | secret}}` 